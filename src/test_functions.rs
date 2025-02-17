use crate::bad_type::BadType;
use proptest::{prop_assert, test_runner::TestCaseResult};

pub fn test_bad_type_pair(input: Vec<(BadType, BadType)>) -> TestCaseResult {
    // Ensure that all elements are sorted correctly.
    for (first, second) in input {
        prop_assert!(first <= second);
    }

    Ok(())
}

pub fn test_bad_type_triple(input: Vec<(BadType, BadType, BadType)>) -> TestCaseResult {
    // Ensure that all elements are sorted correctly.
    for (first, second, third) in input {
        prop_assert!(first <= second);
        prop_assert!(second <= third);
    }

    Ok(())
}
