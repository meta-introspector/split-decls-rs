// Generated macro for SubsetOf (trait)
macro_rules! Depcrate_fmtSubsetOf {
() => {
// Module: crate::fmt
// Provides: {"SubsetOf"}
// Dependencies: {}
# [doc = " Indicates that one format is a subset of another."] # [doc = ""] # [doc = " The subset format can be converted to the superset format"] # [doc = " for free."] pub unsafe trait SubsetOf < Super > : Format where Super : Format , { # [doc = " Validate the *other* direction of conversion; check if"] # [doc = " this buffer from the superset format conforms to the"] # [doc = " subset format."] # [doc = ""] # [doc = " The default calls `Self::validate`, but some conversions"] # [doc = " may implement a check which is cheaper than validating"] # [doc = " from scratch."] fn revalidate_subset (x : & [u8]) -> bool { Self :: validate (x) } }
};
}
