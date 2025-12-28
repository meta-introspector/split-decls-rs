macro_rules! deps {
    () => {
        Visit!();
        Field!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < F > Visit for F where F : FnMut (& Field , & dyn fmt :: Debug) , { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { (self) (field , value) } }
    };
}

impl_120!()