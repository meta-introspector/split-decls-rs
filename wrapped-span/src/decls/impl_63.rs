macro_rules! deps {
    () => {
        Transparency!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Transparency { # [doc = " Returns `true` if the transparency is [`Opaque`]."] # [doc = ""] # [doc = " [`Opaque`]: Transparency::Opaque"] pub fn is_opaque (& self) -> bool { matches ! (self , Self :: Opaque) } }
    };
}

impl_63!();