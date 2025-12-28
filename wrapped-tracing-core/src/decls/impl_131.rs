macro_rules! deps {
    () => {
        Visit!();
        Value!();
        Field!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Value for [u8] { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_bytes (key , self) } }
    };
}

impl_131!()