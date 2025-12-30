// Generated macro for impl_86 (impl)
macro_rules! Depcrate_log_supportimpl_86 {
() => {
// Module: crate::log_support
// Provides: {"impl_86"}
// Dependencies: {}
impl < S : SerializeMap > Visit for SerdeMapVisitorStrippingLog < S > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { if ! Self :: ignore (field) { self . 0 . record_debug (field , value) ; } } fn record_f64 (& mut self , field : & Field , value : f64) { if ! Self :: ignore (field) { self . 0 . record_f64 (field , value) ; } } fn record_i64 (& mut self , field : & Field , value : i64) { if ! Self :: ignore (field) { self . 0 . record_i64 (field , value) ; } } fn record_u64 (& mut self , field : & Field , value : u64) { if ! Self :: ignore (field) { self . 0 . record_u64 (field , value) ; } } fn record_bool (& mut self , field : & Field , value : bool) { if ! Self :: ignore (field) { self . 0 . record_bool (field , value) ; } } fn record_str (& mut self , field : & Field , value : & str) { if ! Self :: ignore (field) { self . 0 . record_str (field , value) ; } } fn record_error (& mut self , field : & Field , value : & (dyn error :: Error + 'static)) { if ! Self :: ignore (field) { self . 0 . record_error (field , value) ; } } }
};
}
