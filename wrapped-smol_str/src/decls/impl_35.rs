macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl From < String > for SmolStr { # [inline (always)] fn from (text : String) -> Self { Self :: new (text) } }
    };
}

impl_35!();