// Generated macro for impl_125 (impl)
macro_rules! Depcrate_yokeimpl_125 {
() => {
// Module: crate::yoke
// Provides: {"impl_125"}
// Dependencies: {}
impl < Y : for < 'a > Yokeable < 'a > , C > Yoke < Y , C > { # [doc = " Helper function allowing one to wrap the cart type `C` in an [`EitherCart`]."] # [doc = ""] # [doc = " This function wraps the cart into the `A` variant. To wrap it into the"] # [doc = " `B` variant, use [`Self::wrap_cart_in_either_b()`]."] # [doc = ""] # [doc = " For an example, see [`EitherCart`]."] # [inline] pub fn wrap_cart_in_either_a < B > (self) -> Yoke < Y , EitherCart < C , B > > { unsafe { self . replace_cart (EitherCart :: A) } } # [doc = " Helper function allowing one to wrap the cart type `C` in an [`EitherCart`]."] # [doc = ""] # [doc = " This function wraps the cart into the `B` variant. To wrap it into the"] # [doc = " `A` variant, use [`Self::wrap_cart_in_either_a()`]."] # [doc = ""] # [doc = " For an example, see [`EitherCart`]."] # [inline] pub fn wrap_cart_in_either_b < A > (self) -> Yoke < Y , EitherCart < A , C > > { unsafe { self . replace_cart (EitherCart :: B) } } }
};
}
