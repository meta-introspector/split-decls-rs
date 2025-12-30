// Generated macro for impl_226 (impl)
macro_rules! Depcrate_errorimpl_226 {
() => {
// Module: crate::error
// Provides: {"impl_226"}
// Dependencies: {}
impl < Src , Dst : ? Sized , A , V > From < SizeError < Src , Dst > > for ConvertError < A , SizeError < Src , Dst > , V > { # [inline (always)] fn from (err : SizeError < Src , Dst >) -> Self { Self :: Size (err) } }
};
}
