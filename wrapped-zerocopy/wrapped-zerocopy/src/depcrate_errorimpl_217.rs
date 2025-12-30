// Generated macro for impl_217 (impl)
macro_rules! Depcrate_errorimpl_217 {
() => {
// Module: crate::error
// Provides: {"impl_217"}
// Dependencies: {}
impl < Src , Dst : ? Sized , S , V > From < AlignmentError < Src , Dst > > for ConvertError < AlignmentError < Src , Dst > , S , V > { # [inline (always)] fn from (err : AlignmentError < Src , Dst >) -> Self { Self :: Alignment (err) } }
};
}
