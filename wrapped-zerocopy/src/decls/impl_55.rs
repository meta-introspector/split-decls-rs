macro_rules! deps {
    () => {
        Wrap!();
        FromBytes!();
        Immutable!();
        IntoBytes!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'a , Src , Dst > Wrap < & 'a Src , & 'a Dst > { # [doc = " # Safety"] # [doc = " The caller must guarantee that:"] # [doc = " - `Src: IntoBytes + Immutable`"] # [doc = " - `Dst: FromBytes + Immutable`"] # [doc = ""] # [doc = " # PME"] # [doc = ""] # [doc = " Instantiating this method PMEs unless both:"] # [doc = " - `mem::size_of::<Dst>() == mem::size_of::<Src>()`"] # [doc = " - `mem::align_of::<Dst>() <= mem::align_of::<Src>()`"] # [inline (always)] # [must_use] pub const unsafe fn transmute_ref (self) -> & 'a Dst { static_assert ! (Src , Dst => mem :: size_of ::< Dst > () == mem :: size_of ::< Src > ()) ; static_assert ! (Src , Dst => mem :: align_of ::< Dst > () <= mem :: align_of ::< Src > ()) ; let src : * const Src = self . 0 ; let dst = src . cast :: < Dst > () ; # [allow (clippy :: transmute_ptr_to_ref)] unsafe { mem :: transmute (dst) } } }
    };
}

impl_55!()