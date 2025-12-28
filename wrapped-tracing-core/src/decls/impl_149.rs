macro_rules! deps {
    () => {
        Field!();
        Visit!();
        Value!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Value for String { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_str (key , self . as_str ()) } }
    };
}

impl_149!();