macro_rules! deps {
    () => {
        Wrap!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < Src , Dst > Wrap < Src , Dst > { # [inline (always)] pub const fn new (src : Src) -> Self { Wrap (src , PhantomData) } }
    };
}

impl_54!();