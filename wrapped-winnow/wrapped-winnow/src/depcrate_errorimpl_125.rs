// Generated macro for impl_125 (impl)
macro_rules! Depcrate_errorimpl_125 {
() => {
// Module: crate::error
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > ErrorConvert < TreeError < (I , usize) , C > > for TreeError < I , C > { # [inline] fn convert (self) -> TreeError < (I , usize) , C > { self . map_input (| i | (i , 0)) } }
};
}
