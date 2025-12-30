// Generated macro for impl_82 (impl)
macro_rules! Depcrate_errorimpl_82 {
() => {
// Module: crate::error
// Provides: {"impl_82"}
// Dependencies: {}
impl < I : Clone > ErrorConvert < InputError < I > > for InputError < (I , usize) > { # [inline] fn convert (self) -> InputError < I > { self . map_input (| (i , _o) | i) } }
};
}
