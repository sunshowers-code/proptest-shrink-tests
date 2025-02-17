# proptest-shrink-tests

Profiling for [proptest](https://proptest-rs.github.io/proptest/intro.html) shrinking. See my accompanying blog post: [*Demystifying monads in Rust through property-based testing*](https://sunshowers.io/posts/monads-through-pbt).

## Running the code

Use `cargo run` to run in dev mode with opt-level 1 (typical configuration for testing), or `cargo run --release` to run in release mode with opt-level 3. This will write out the raw data to `results-opt-level-N.tsv`, where `N` is the opt-level value.

## Plotting charts

Run `./plot-cdfs.gnu`. You'll need gnuplot and imagemagick installed.

## Results

On my workstation (Ryzen 7950X running Linux 6.12 with Rust 1.84.1), with opt-level 1, the amount of time it takes is:

| Metric  | Pairs (`prop_map`) | Triples (`prop_map`) | Pairs (`prop_flat_map`) | Triples (`prop_flat_map`) |
|:-------:|----------------------:|------------------------:|----------------------:|------------------------:|
| **min** |                 11 µs |                   48 µs |               3.85 ms |                 8.95 ms |
| **p50** |               1.70 ms |                 2.52 ms |               8.52 ms |                  181 ms |
| **p75** |               3.74 ms |                 5.77 ms |              10.04 ms |                  307 ms |
| **p90** |               5.25 ms |                 8.41 ms |              11.76 ms |                  435 ms |
| **max** |               7.00 ms |                10.55 ms |              15.53 ms |                 1808 ms |

The number of iterations:

| Metric  | Pairs (`prop_map`) | Triples (`prop_map`) | Pairs (`prop_flat_map`) | Triples (`prop_flat_map`) |
|:-------:|----------------------:|------------------------:|----------------------:|------------------------:|
| **min** |                    48 |                      93 |                  1228 |                   11223 |
| **p50** |                   215 |                     306 |                  6722 |                  281016 |
| **p75** |                   270 |                     354 |                  9315 |                  481996 |
| **p90** |                   310 |                     410 |                 10722 |                  693358 |
| **max** |                   387 |                     530 |                 12242 |                  884729 |

A [CDF](https://en.wikipedia.org/wiki/Cumulative_distribution_function) of results:

![There are two log‐log scale CDF (cumulative distribution function) plots, each showing four lines labeled “pair map,” “triple map,” “pair flat_map,” and “triple flat_map.” For the top plot (cdf of shrink execution time), the x‐axis ranges roughly from 10 µs to 1 × 10^6 µs (log scale) and the y‐axis shows cumulative probability from 0.01 to 1.0 (also log scale). The “pair map” (green) and “triple map” (purple) curves overlap around 100 µs to about 1,000 µs, reaching 100% probability before the “pair flat_map” (blue) and “triple flat_map” (orange) lines. The blue line peaks around tens of thousands of microseconds, while the orange line extends further toward 1 × 10^6 µs before leveling off. For the bottom plot (cdf of number of shrink iterations), the x‐axis is the number of iterations (10 to 1 × 10^6 on a log scale) and the y‐axis is cumulative probability (0.01 to 1.0 on a log scale). Again, “pair map” (green) and “triple map” (purple) are at lower iteration counts (roughly tens to hundreds) and reach 100% probability faster. “Pair flat_map” (blue) extends to thousands of iterations, and “triple flat_map” (orange) continues to tens or hundreds of thousands of iterations before reaching 100%. A legend in the top‐right corner identifies each line’s label. In the bottom right is system information (Ryzen 7950X, Linux 6.12, Rust 1.84.1, opt‐level 1).](performance_cdf.png)

For more discussion, see [the *Measuring the impact* section](https://sunshowers.io/posts/monads-through-pbt#measuring-the-impact) in my blog post.
