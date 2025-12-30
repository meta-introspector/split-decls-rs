// Generated macro for impl_166 (impl)
macro_rules! Depcrate_tupleimpl_166 {
() => {
// Module: crate::tuple
// Provides: {"impl_166"}
// Dependencies: {}
impl < T1 : TypeSize , T2 : TypeSize > TypeSize for (T1 , T2) { fn extra_size (& self) -> usize { self . 0 . extra_size () + self . 1 . extra_size () } }
};
}
