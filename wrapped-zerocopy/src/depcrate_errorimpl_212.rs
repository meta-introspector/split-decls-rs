// Generated macro for impl_212 (impl)
macro_rules! Depcrate_errorimpl_212 {
() => {
// Module: crate::error
// Provides: {"impl_212"}
// Dependencies: {}
impl < Src , Dst : ? Sized + Unaligned > From < AlignmentError < Src , Dst > > for Infallible { # [inline (always)] fn from (_ : AlignmentError < Src , Dst >) -> Infallible { unsafe { core :: hint :: unreachable_unchecked () } } }
};
}
