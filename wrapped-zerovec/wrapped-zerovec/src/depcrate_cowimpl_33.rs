// Generated macro for impl_33 (impl)
macro_rules! Depcrate_cowimpl_33 {
() => {
// Module: crate::cow
// Provides: {"impl_33"}
// Dependencies: {}
# [cfg (feature = "databake")] impl < 'a , V : VarULE + ? Sized > databake :: BakeSize for VarZeroCow < 'a , V > { fn borrows_size (& self) -> usize { self . as_bytes () . len () } }
};
}
