macro_rules! deps {
    () => {
        Value!();
        Field!();
        Visit!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl Value for fmt :: Arguments < '_ > { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_debug (key , self) } }
    };
}

impl_145!();