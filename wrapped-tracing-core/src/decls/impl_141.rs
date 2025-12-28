macro_rules! deps {
    () => {
        Value!();
        Visit!();
        Field!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Value for & 'a T where T : Value + 'a , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { (* self) . record (key , visitor) } }
    };
}

impl_141!();