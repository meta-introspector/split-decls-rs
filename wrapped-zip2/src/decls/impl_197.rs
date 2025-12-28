macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl From < DateTime > for (u16 , u16) { # [inline] fn from (dt : DateTime) -> Self { (dt . datepart () , dt . timepart ()) } }
    };
}

impl_197!()