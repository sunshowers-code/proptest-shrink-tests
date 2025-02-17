use proptest::{
    prelude::*,
    strategy::ValueTree,
    test_runner::{noop_result_cache, ResultCache, TestCaseError, TestCaseResult, TestRunner},
};
use std::{
    fmt,
    io::Write,
    panic::{self, AssertUnwindSafe},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct ShrinkMetrics {
    pub iterations: u64,
    pub time_taken: Duration,
}

#[derive(Debug)]
pub struct Results {
    pub name: &'static str,
    pub successes: usize,
    pub shrink_metrics: Vec<ShrinkMetrics>,
}

impl Results {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            successes: 0,
            shrink_metrics: Vec::new(),
        }
    }
}

pub fn run_all<S: Strategy>(
    name: &'static str,
    total: usize,
    strategy: &S,
    test: fn(S::Value) -> Result<(), TestCaseError>,
) -> Results {
    let mut test_runner = TestRunner::deterministic();

    // noop_result_cache is the default cache implementation for proptest.
    let mut result_cache = noop_result_cache();
    let mut results = Results::new(name);

    let mut n = 0;

    while n < total {
        let value_tree = strategy
            .new_tree(&mut test_runner)
            .expect("value generated");

        let result = run_one(name, n + 1, total, test, value_tree, &mut *result_cache);

        match result {
            Ok(()) => {
                results.successes += 1;
            }
            Err(metrics) => {
                results.shrink_metrics.push(metrics);
                n += 1;
            }
        }
    }

    results
}

/// Run a single test case, shrinking it if required.
pub fn run_one<V: ValueTree>(
    name: &str,
    current: usize,
    total: usize,
    test: fn(V::Value) -> Result<(), TestCaseError>,
    mut value_tree: V,
    result_cache: &mut dyn ResultCache,
) -> Result<(), ShrinkMetrics> {
    // Run the test.
    match call_test(value_tree.current(), &test, result_cache) {
        Ok(()) => {
            eprintln!("[{name} {current}/{total}] test passed");
            Ok(())
        }
        Err(err) => {
            eprintln!(
                "[{name} {current}/{total}] test failed, starting to shrink: {}",
                err
            );
            let metrics = shrink(&mut value_tree, test, result_cache);
            eprintln!("shrink metrics: {:?}", metrics);
            Err(metrics)
        }
    }
}

pub fn write_tsv(all_results: &[Results], output: &mut dyn Write) -> color_eyre::Result<()> {
    // Column headers.
    write!(output, "# ")?;
    for result in all_results {
        // Two columns: time taken, number of iterations
        write!(
            output,
            "{}_time_taken_micros\t{}_iterations\t",
            result.name, result.name
        )?;
    }
    writeln!(output)?;

    // Data rows.
    let row_count = all_results[0].shrink_metrics.len();
    for row_index in 0..row_count {
        for all_result in all_results {
            let row = &all_result.shrink_metrics[row_index];
            write!(
                output,
                "{}\t{}\t",
                row.time_taken.as_micros(),
                row.iterations
            )?;
        }
        writeln!(output)?;
    }

    Ok(())
}

// This function is adapted from the "shrink" function in proptest. Ordinarily,
// proptest takes care of this for you, but here we reimplement shrinking to
// collect metrics.
fn shrink<V: ValueTree>(
    case: &mut V,
    test: impl Fn(V::Value) -> TestCaseResult,
    result_cache: &mut dyn ResultCache,
) -> ShrinkMetrics {
    let start_time = Instant::now();
    let mut iterations = 0;

    if case.simplify() {
        loop {
            iterations += 1;

            // Call the test function with the current case value.
            let result = call_test(case.current(), &test, result_cache);

            match result {
                // Rejections are effectively a pass here,
                // since they indicate that any behaviour of
                // the function under test is acceptable.
                Ok(_) | Err(TestCaseError::Reject(..)) => {
                    if !case.complicate() {
                        break;
                    }
                }
                Err(TestCaseError::Fail(_)) => {
                    if !case.simplify() {
                        break;
                    }
                }
            }
        }
    }

    ShrinkMetrics {
        iterations,
        time_taken: start_time.elapsed(),
    }
}

fn call_test<V, F>(case: V, test: &F, _result_cache: &mut dyn ResultCache) -> TestCaseResult
where
    V: fmt::Debug,
    F: Fn(V) -> TestCaseResult,
{
    match panic::catch_unwind(AssertUnwindSafe(|| test(case))) {
        Ok(result) => result,
        Err(what) => Err(TestCaseError::Fail(
            what.downcast::<&'static str>()
                .map(|s| (*s).into())
                .or_else(|what| what.downcast::<String>().map(|b| (*b).into()))
                .or_else(|what| what.downcast::<Box<str>>().map(|b| (*b).into()))
                .unwrap_or_else(|_| "<unknown panic value>".into()),
        )),
    }
}
