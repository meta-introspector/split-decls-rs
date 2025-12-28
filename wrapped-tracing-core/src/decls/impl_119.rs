macro_rules! deps {
    () => {
        Field!();
        Visit!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl Visit for fmt :: DebugMap < '_ , '_ > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . entry (& format_args ! ("{}" , field) , value) ; } }
    };
}

impl_119!()