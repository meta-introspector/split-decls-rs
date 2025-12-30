// Generated macro for impl_155 (impl)
macro_rules! Depcrate_fieldimpl_155 {
() => {
// Module: crate::field
// Provides: {"impl_155"}
// Dependencies: {}
impl Visit for fmt :: DebugMap < '_ , '_ > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . entry (& format_args ! ("{}" , field) , value) ; } }
};
}
