// Generated macro for impl_107 (impl)
macro_rules! Depcrate_yokeimpl_107 {
() => {
// Module: crate::yoke
// Provides: {"impl_107"}
// Dependencies: {}
impl < Y : for < 'a > Yokeable < 'a > > Yoke < Y , () > { # [doc = " Construct a new [`Yoke`] from static data. There will be no"] # [doc = " references to `cart` here since [`Yokeable`]s are `'static`,"] # [doc = " this is good for e.g. constructing fully owned"] # [doc = " [`Yoke`]s with no internal borrowing."] # [doc = ""] # [doc = " This is similar to [`Yoke::new_owned()`] but it does not allow you to"] # [doc = " mix the [`Yoke`] with borrowed data. This is primarily useful"] # [doc = " for using [`Yoke`] in generic scenarios."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use yoke::Yoke;"] # [doc = " # use std::borrow::Cow;"] # [doc = ""] # [doc = " let owned: Cow<str> = \"hello\".to_owned().into();"] # [doc = " // this yoke can be intermingled with actually-borrowed Yokes"] # [doc = " let yoke: Yoke<Cow<str>, ()> = Yoke::new_always_owned(owned);"] # [doc = ""] # [doc = " assert_eq!(yoke.get(), \"hello\");"] # [doc = " ```"] pub fn new_always_owned (yokeable : Y) -> Self { Self { yokeable : KindaSortaDangling :: new (yokeable) , cart : () , } } # [doc = " Obtain the yokeable out of a `Yoke<Y, ()>`"] # [doc = ""] # [doc = " For most `Yoke` types this would be unsafe but it's"] # [doc = " fine for `Yoke<Y, ()>` since there are no actual internal"] # [doc = " references"] pub fn into_yokeable (self) -> Y { self . yokeable . into_inner () } }
};
}
