macro_rules! deps {
    () => {
        Field!();
        DebugValue!();
        Visit!();
        Value!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T > Value for DebugValue < T > where T : fmt :: Debug , { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_debug (key , & self . 0) } }
    };
}

impl_157!();