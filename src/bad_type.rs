use proptest::prelude::*;
use std::cell::RefCell;
use test_strategy::Arbitrary;

#[derive(Clone, Copy, Debug, Arbitrary)]
pub enum OrdBehavior {
    // Normal Ord behavior.
    #[weight(4)]
    Regular,

    // Reversed behavior.
    #[weight(1)]
    Flipped,
}

#[derive(Clone, Debug, Arbitrary)]
pub struct BadType {
    #[strategy(0..10000u64)]
    pub value: u64,
    #[strategy(prop::collection::vec(OrdBehavior::arbitrary(), 0..128).prop_map(|mut v| { v.reverse(); RefCell::new(v) }))]
    pub ord_behavior: RefCell<Vec<OrdBehavior>>,
}

impl Ord for BadType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.ord_behavior.borrow_mut().pop() {
            Some(OrdBehavior::Regular) | None => self.value.cmp(&other.value),
            Some(OrdBehavior::Flipped) => other.value.cmp(&self.value),
        }
    }
}

impl PartialOrd for BadType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BadType {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for BadType {}

// First, a strategy for OrdBehavior instances.
pub fn generate_ord_behavior() -> impl Strategy<Value = OrdBehavior> {
    prop_oneof![
        // 4/5 chance that the Regular implementation is generated.
        4 => Just(OrdBehavior::Regular),
        // 1/5 chance that it's flipped.
        1 => Just(OrdBehavior::Flipped),
    ]
}

// To generate a BadType:
pub fn generate_bad_type() -> impl Strategy<Value = BadType> {
    // Use the range strategy to generate values uniformly at random.
    let value_strategy = 0..10000_u64;

    // Use the vec strategy to generate a list of behaviors, 0..128 items long.
    let ord_behavior_strategy = prop::collection::vec(generate_ord_behavior(), 0..128);

    // Now what? We need to compose these strategies together. With proptest,
    // the way to do this is to first create a tuple of strategies.
    let tuple_strategy = (value_strategy, ord_behavior_strategy);

    // A tuple of strategies is also a strategy! Generated values are a tuple
    // of constituents.
    //
    // Now we can use a function called `prop_map` to turn the tuple into
    // a BadType.
    tuple_strategy.prop_map(|(value, ord_behavior)| BadType {
        value,
        ord_behavior: RefCell::new(ord_behavior),
    })
}

/// Generate a pair of BadType instances, where the second value is greater than
/// the first, using prop_flat_map (monadic composition).
pub fn generate_bad_type_pair_flat_map() -> impl Strategy<Value = (BadType, BadType)> {
    // First generate a BadType.
    generate_bad_type().prop_flat_map(|bad1| {
        // Now generate a second BadType with a value greater than the first.
        (
            (bad1.value + 1)..20000_u64,
            prop::collection::vec(generate_ord_behavior(), 0..128),
        )
            .prop_map(move |(second_value, ord_behavior)| {
                // Generate the second value.
                let second_bad_type = BadType {
                    value: bad1.value + second_value,
                    ord_behavior: RefCell::new(ord_behavior),
                };

                // Return the pair.
                (bad1.clone(), second_bad_type)
            })
    })
}

/// Generate a triple of BadType instances in a monadic fashion.
pub fn generate_bad_type_triple_flat_map() -> impl Strategy<Value = (BadType, BadType, BadType)> {
    // Generate three BadType instances.
    generate_bad_type_pair_flat_map().prop_flat_map(|(bad1, bad2)| {
        // Generate a third BadType with a value greater than the second.
        (
            (bad2.value + 1)..30000_u64,
            prop::collection::vec(generate_ord_behavior(), 0..128),
        )
            .prop_map(move |(third_value, ord_behavior)| {
                // Generate the third value.
                let third_bad_type = BadType {
                    value: bad2.value + third_value,
                    ord_behavior: RefCell::new(ord_behavior),
                };

                // Return the triple.
                (bad1.clone(), bad2.clone(), third_bad_type)
            })
    })
}

/// Generate a pair of BadType instances, where the second value is greater than
/// the first, using prop_map (non-monadic composition).
pub fn generate_bad_type_pair_map() -> impl Strategy<Value = (BadType, BadType)> {
    // Generate two BadType instances.
    (generate_bad_type(), generate_bad_type())
        // Look, no prop_flat_map! This is non-monadic composition.
        .prop_map(|(bad1, mut bad2)| {
            // Add bad1.value to bad2.value. Because the two are non-negative
            // (unsigned integers), this ensures that bad2.value is always
            // bigger than bad1.value.
            bad2.value += bad1.value;
            (bad1, bad2)
        })
}

/// Generate a triple of `BadType` instances using `prop_map`.
pub fn generate_bad_type_triple_map() -> impl Strategy<Value = (BadType, BadType, BadType)> {
    // Generate two BadType instances.
    (
        generate_bad_type(),
        generate_bad_type(),
        generate_bad_type(),
    )
        // Look, no prop_flat_map! This is non-monadic composition.
        .prop_map(|(bad1, mut bad2, mut bad3)| {
            // Add bad1.value to bad2.value. Because the two are non-negative
            // (unsigned integers), this ensures that bad2.value is always
            // bigger than bad1.value.
            bad2.value += bad1.value;
            bad3.value += bad2.value;
            (bad1, bad2, bad3)
        })
}
