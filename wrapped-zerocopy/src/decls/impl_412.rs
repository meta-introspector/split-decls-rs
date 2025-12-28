macro_rules! deps {
    () => {
        Unalign!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < T : Copy > Unalign < T > { # [doc = " Gets a copy of the inner `T`."] # [inline (always)] pub fn get (& self) -> T { let Unalign (val) = * self ; val } }
    };
}

impl_412!();