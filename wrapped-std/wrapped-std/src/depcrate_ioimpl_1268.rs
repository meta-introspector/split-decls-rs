// Generated macro for impl_1268 (impl)
macro_rules! Depcrate_ioimpl_1268 {
() => {
// Module: crate::io
// Provides: {"impl_1268"}
// Dependencies: {}
impl < T > SizeHint for Box < T > { # [inline] fn lower_bound (& self) -> usize { SizeHint :: lower_bound (& * * self) } # [inline] fn upper_bound (& self) -> Option < usize > { SizeHint :: upper_bound (& * * self) } }
};
}
