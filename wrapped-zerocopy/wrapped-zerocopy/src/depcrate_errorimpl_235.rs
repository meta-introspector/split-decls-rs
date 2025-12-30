// Generated macro for impl_235 (impl)
macro_rules! Depcrate_errorimpl_235 {
() => {
// Module: crate::error
// Provides: {"impl_235"}
// Dependencies: {}
impl < Src , Dst : ? Sized + TryFromBytes , A , S > From < ValidityError < Src , Dst > > for ConvertError < A , S , ValidityError < Src , Dst > > { # [inline (always)] fn from (err : ValidityError < Src , Dst >) -> Self { Self :: Validity (err) } }
};
}
