// Generated macro for impl_108 (impl)
macro_rules! Depcrate_yokeimpl_108 {
() => {
// Module: crate::yoke
// Provides: {"impl_108"}
// Dependencies: {}
impl < Y : for < 'a > Yokeable < 'a > , C > Yoke < Y , Option < C > > { # [doc = " Construct a new [`Yoke`] from static data. There will be no"] # [doc = " references to `cart` here since [`Yokeable`]s are `'static`,"] # [doc = " this is good for e.g. constructing fully owned"] # [doc = " [`Yoke`]s with no internal borrowing."] # [doc = ""] # [doc = " This can be paired with [`Yoke:: wrap_cart_in_option()`] to mix owned"] # [doc = " and borrowed data."] # [doc = ""] # [doc = " If you do not wish to pair this with borrowed data, [`Yoke::new_always_owned()`] can"] # [doc = " be used to get a [`Yoke`] API on always-owned data."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use yoke::Yoke;"] # [doc = " # use std::borrow::Cow;"] # [doc = " # use std::rc::Rc;"] # [doc = ""] # [doc = " let owned: Cow<str> = \"hello\".to_owned().into();"] # [doc = " // this yoke can be intermingled with actually-borrowed Yokes"] # [doc = " let yoke: Yoke<Cow<str>, Option<Rc<[u8]>>> = Yoke::new_owned(owned);"] # [doc = ""] # [doc = " assert_eq!(yoke.get(), \"hello\");"] # [doc = " ```"] pub const fn new_owned (yokeable : Y) -> Self { Self { yokeable : KindaSortaDangling :: new (yokeable) , cart : None , } } # [doc = " Obtain the yokeable out of a `Yoke<Y, Option<C>>` if possible."] # [doc = ""] # [doc = " If the cart is `None`, this returns `Ok`, but if the cart is `Some`,"] # [doc = " this returns `self` as an error."] pub fn try_into_yokeable (self) -> Result < Y , Self > { match self . cart { Some (_) => Err (self) , None => Ok (self . yokeable . into_inner ()) , } } }
};
}
