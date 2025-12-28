macro_rules! deps {
    () => {
        Field!();
        Visit!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Visit for fmt :: DebugStruct < '_ , '_ > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . field (field . name () , value) ; } }
    };
}

impl_118!()