macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl ops :: Deref for SmolStr { type Target = str ; # [inline (always)] fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_5!()