use proptest::prelude::*;

proptest! {
    #[test]
    fn property_harness_runs(x in 0u8..4, y in 0u8..4) {
        prop_assert_eq!(x + y, y + x);
    }
}
