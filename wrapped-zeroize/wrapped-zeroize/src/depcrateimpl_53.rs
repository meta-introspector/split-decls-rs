// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < Z > Zeroize for Vec < Z > where Z : Zeroize , { # [doc = " \"Best effort\" zeroization for `Vec`."] # [doc = ""] # [doc = " Ensures the entire capacity of the `Vec` is zeroed. Cannot ensure that"] # [doc = " previous reallocations did not leave values on the heap."] fn zeroize (& mut self) { self . iter_mut () . zeroize () ; self . clear () ; self . spare_capacity_mut () . zeroize () ; } }
};
}
