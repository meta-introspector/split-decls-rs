macro_rules! deps {
    () => {
        Messages!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < V > Visit for Messages < V > where V : Visit , { # [inline] fn record_f64 (& mut self , field : & Field , value : f64) { self . 0 . record_f64 (field , value) } # [inline] fn record_i64 (& mut self , field : & Field , value : i64) { self . 0 . record_i64 (field , value) } # [inline] fn record_u64 (& mut self , field : & Field , value : u64) { self . 0 . record_u64 (field , value) } # [inline] fn record_bool (& mut self , field : & Field , value : bool) { self . 0 . record_bool (field , value) } # [doc = " Visit a string value."] fn record_str (& mut self , field : & Field , value : & str) { if field . name () == "message" { self . 0 . record_debug (field , & format_args ! ("{}" , value)) } else { self . 0 . record_str (field , value) } } # [inline] fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . 0 . record_debug (field , value) } }
    };
}

impl_26!();