macro_rules! deps {
    () => {
        VisitDelimited!();
        VisitFmt!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < D , V > Visit for VisitDelimited < D , V > where V : VisitFmt , D : AsRef < str > , { fn record_i64 (& mut self , field : & Field , value : i64) { self . delimit () ; self . inner . record_i64 (field , value) ; } fn record_u64 (& mut self , field : & Field , value : u64) { self . delimit () ; self . inner . record_u64 (field , value) ; } fn record_bool (& mut self , field : & Field , value : bool) { self . delimit () ; self . inner . record_bool (field , value) ; } fn record_str (& mut self , field : & Field , value : & str) { self . delimit () ; self . inner . record_str (field , value) ; } fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . delimit () ; self . inner . record_debug (field , value) ; } }
    };
}

impl_18!();