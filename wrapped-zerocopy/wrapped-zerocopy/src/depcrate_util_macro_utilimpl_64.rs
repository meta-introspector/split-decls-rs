// Generated macro for impl_64 (impl)
macro_rules! Depcrate_util_macro_utilimpl_64 {
() => {
// Module: crate::util::macro_util
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , Src , Dst > Wrap < & 'a mut Src , & 'a mut Dst > { # [doc = " Transmutes a mutable reference of one type to a mutable reference of another"] # [doc = " type."] # [doc = ""] # [doc = " # PME"] # [doc = ""] # [doc = " Instantiating this method PMEs unless both:"] # [doc = " - `mem::size_of::<Dst>() == mem::size_of::<Src>()`"] # [doc = " - `mem::align_of::<Dst>() <= mem::align_of::<Src>()`"] # [inline (always)] # [must_use] pub fn transmute_mut (self) -> & 'a mut Dst where Src : FromBytes + IntoBytes , Dst : FromBytes + IntoBytes , { static_assert ! (Src , Dst => mem :: size_of ::< Dst > () == mem :: size_of ::< Src > ()) ; static_assert ! (Src , Dst => mem :: align_of ::< Dst > () <= mem :: align_of ::< Src > ()) ; let src : * mut Src = self . 0 ; let dst = src . cast :: < Dst > () ; unsafe { & mut * dst } } }
};
}
