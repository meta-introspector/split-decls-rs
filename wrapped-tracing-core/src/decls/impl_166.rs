macro_rules! deps {
    () => {
        Value!();
        Field!();
        Visit!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T : Value > Value for Option < T > { fn record (& self , key : & Field , visitor : & mut dyn Visit) { if let Some (v) = & self { v . record (key , visitor) } } }
    };
}

impl_166!();