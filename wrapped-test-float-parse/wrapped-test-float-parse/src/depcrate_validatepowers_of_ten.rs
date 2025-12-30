// Generated macro for POWERS_OF_TEN (static)
macro_rules! Depcrate_validatePOWERS_OF_TEN {
() => {
// Module: crate::validate
// Provides: {"POWERS_OF_TEN"}
// Dependencies: {}
# [doc = " Cached powers of 10 so we can look them up rather than recreating."] static POWERS_OF_TEN : LazyLock < BTreeMap < i32 , BigRational > > = LazyLock :: new (| | { POWERS_OF_TEN_RANGE . map (| exp | (exp , BigRational :: from_u32 (10) . unwrap () . pow (exp))) . collect () }) ;
};
}
