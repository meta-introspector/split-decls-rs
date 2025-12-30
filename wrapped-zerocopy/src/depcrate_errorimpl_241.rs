// Generated macro for impl_241 (impl)
macro_rules! Depcrate_errorimpl_241 {
() => {
// Module: crate::error
// Provides: {"impl_241"}
// Dependencies: {}
impl < Src , Dst : ? Sized + TryFromBytes > From < CastError < Src , Dst > > for TryCastError < Src , Dst > { # [inline] fn from (value : CastError < Src , Dst >) -> Self { match value { CastError :: Alignment (e) => Self :: Alignment (e) , CastError :: Size (e) => Self :: Size (e) , CastError :: Validity (i) => match i { } , } } }
};
}
