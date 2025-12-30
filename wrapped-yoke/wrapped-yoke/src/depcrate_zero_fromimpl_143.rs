// Generated macro for impl_143 (impl)
macro_rules! Depcrate_zero_fromimpl_143 {
() => {
// Module: crate::zero_from
// Provides: {"impl_143"}
// Dependencies: {}
impl < Y , C > Yoke < Y , C > where Y : for < 'a > Yokeable < 'a > , for < 'a > < Y as Yokeable < 'a > > :: Output : ZeroFrom < 'a , < C as Deref > :: Target > , C : StableDeref + Deref , < C as Deref > :: Target : 'static , { # [doc = " Construct a [`Yoke`]`<Y, C>` from a cart implementing `StableDeref` by zero-copy cloning"] # [doc = " the cart to `Y` and then yokeing that object to the cart."] # [doc = ""] # [doc = " The type `Y` must implement [`ZeroFrom`]`<C::Target>`. This trait is auto-implemented"] # [doc = " on many common types and can be custom implemented or derived in order to make it easier"] # [doc = " to construct a `Yoke`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Attach to a cart:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::borrow::Cow;"] # [doc = " use yoke::Yoke;"] # [doc = ""] # [doc = " let yoke = Yoke::<Cow<'static, str>, String>::attach_to_zero_copy_cart("] # [doc = "     \"demo\".to_owned(),"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!(\"demo\", yoke.get());"] # [doc = " ```"] pub fn attach_to_zero_copy_cart (cart : C) -> Self { Yoke :: < Y , C > :: attach_to_cart (cart , | c | < Y as Yokeable > :: Output :: zero_from (c)) } }
};
}
