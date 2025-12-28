macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl AsRef < Uuid > for Uuid { # [inline] fn as_ref (& self) -> & Uuid { self } }
    };
}

impl_143!()