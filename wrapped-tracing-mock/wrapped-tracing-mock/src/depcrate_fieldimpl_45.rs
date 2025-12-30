// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fieldimpl_45 {
() => {
// Module: crate::field
// Provides: {"impl_45"}
// Dependencies: {}
impl Visit for CheckVisitor < '_ > { fn record_f64 (& mut self , field : & Field , value : f64) { self . expect . compare_or_panic (field . name () , & value , self . ctx , self . subscriber_name) } fn record_i64 (& mut self , field : & Field , value : i64) { self . expect . compare_or_panic (field . name () , & value , self . ctx , self . subscriber_name) } fn record_u64 (& mut self , field : & Field , value : u64) { self . expect . compare_or_panic (field . name () , & value , self . ctx , self . subscriber_name) } fn record_bool (& mut self , field : & Field , value : bool) { self . expect . compare_or_panic (field . name () , & value , self . ctx , self . subscriber_name) } fn record_str (& mut self , field : & Field , value : & str) { self . expect . compare_or_panic (field . name () , & value , self . ctx , self . subscriber_name) } fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . expect . compare_or_panic (field . name () , & field :: debug (value) , self . ctx , self . subscriber_name ,) } }
};
}
