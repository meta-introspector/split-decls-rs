// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl From < Choice > for bool { # [doc = " Convert the `Choice` wrapper into a `bool`, depending on whether"] # [doc = " the underlying `u8` was a `0` or a `1`."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function exists to avoid having higher-level cryptographic protocol"] # [doc = " implementations duplicating this pattern."] # [doc = ""] # [doc = " The intended use case for this conversion is at the _end_ of a"] # [doc = " higher-level primitive implementation: for example, in checking a keyed"] # [doc = " MAC, where the verification should happen in constant-time (and thus use"] # [doc = " a `Choice`) but it is safe to return a `bool` at the end of the"] # [doc = " verification."] # [inline] fn from (source : Choice) -> bool { debug_assert ! ((source . 0 == 0u8) | (source . 0 == 1u8)) ; source . 0 != 0 } }
};
}
