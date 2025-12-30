// Generated macro for impl_110 (impl)
macro_rules! Depcrate_yokeimpl_110 {
() => {
// Module: crate::yoke
// Provides: {"impl_110"}
// Dependencies: {}
impl < Y : for < 'a > Yokeable < 'a > , C : CartablePointerLike > Yoke < Y , CartableOptionPointer < C > > { # [doc = " Obtain the yokeable out of a `Yoke<Y, CartableOptionPointer<C>>` if possible."] # [doc = ""] # [doc = " If the cart is `None`, this returns `Ok`, but if the cart is `Some`,"] # [doc = " this returns `self` as an error."] # [inline] pub fn try_into_yokeable (self) -> Result < Y , Self > { if self . cart . is_none () { Ok (self . yokeable . into_inner ()) } else { Err (self) } } }
};
}
