macro_rules! deps {
    () => {
        Value!();
        Visit!();
        Field!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > Value for & 'a mut T where T : Value + 'a , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { T :: record (self , key , visitor) } }
    };
}

impl_143!();