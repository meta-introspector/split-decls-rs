// Generated macro for impl_81 (impl)
macro_rules! Depcrate_errorimpl_81 {
() => {
// Module: crate::error
// Provides: {"impl_81"}
// Dependencies: {}
impl < I : Clone > ErrorConvert < InputError < (I , usize) > > for InputError < I > { # [inline] fn convert (self) -> InputError < (I , usize) > { self . map_input (| i | (i , 0)) } }
};
}
