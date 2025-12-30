// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl Visit for Fields { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . record (field , format ! ("{:?}" , value)) ; } fn record_f64 (& mut self , field : & Field , value : f64) { self . record (field , value) ; } fn record_i64 (& mut self , field : & Field , value : i64) { self . record (field , value) ; } fn record_u64 (& mut self , field : & Field , value : u64) { self . record (field , value) ; } fn record_bool (& mut self , field : & Field , value : bool) { self . record (field , value) ; } fn record_str (& mut self , field : & Field , value : & str) { self . record (field , value) ; } fn record_error (& mut self , field : & Field , value : & (dyn error :: Error + 'static)) { self . record (field , format ! ("{}" , value)) ; } }
};
}
