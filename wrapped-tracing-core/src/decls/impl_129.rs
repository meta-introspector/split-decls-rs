macro_rules! deps {
    () => {
        Value!();
        Field!();
        Visit!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Value for str { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_str (key , self) } }
    };
}

impl_129!();