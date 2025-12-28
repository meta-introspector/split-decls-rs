macro_rules! deps {
    () => {
        FmtEvent!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > Visit for FmtEvent < 'a > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { let buf = & mut self . bufs . current_buf ; let comma = if self . comma { "," } else { "" } ; match field . name () { "message" => { write ! (buf , "{} {:?}" , comma , value) . unwrap () ; self . comma = true ; } # [cfg (feature = "tracing-log")] name if name . starts_with ("log.") => { } name => { write ! (buf , "{} {}={:?}" , comma , name , value) . unwrap () ; self . comma = true ; } } } }
    };
}

impl_14!();