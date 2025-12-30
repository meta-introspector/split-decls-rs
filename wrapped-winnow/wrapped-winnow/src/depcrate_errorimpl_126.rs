// Generated macro for impl_126 (impl)
macro_rules! Depcrate_errorimpl_126 {
() => {
// Module: crate::error
// Provides: {"impl_126"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > ErrorConvert < TreeError < I , C > > for TreeError < (I , usize) , C > { # [inline] fn convert (self) -> TreeError < I , C > { self . map_input (| (i , _o) | i) } }
};
}
