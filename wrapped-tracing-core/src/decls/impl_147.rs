macro_rules! deps {
    () => {
        Visit!();
        Value!();
        Field!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < T : ? Sized > Value for Box < T > where T : Value , { # [inline] fn record (& self , key : & Field , visitor : & mut dyn Visit) { self . as_ref () . record (key , visitor) } }
    };
}

impl_147!();