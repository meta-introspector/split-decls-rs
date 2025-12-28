macro_rules! deps {
    () => {
        Field!();
        DisplayValue!();
        Value!();
        Visit!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T > Value for DisplayValue < T > where T : fmt :: Display , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_debug (key , self) } }
    };
}

impl_153!()