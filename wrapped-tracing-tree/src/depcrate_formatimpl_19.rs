// Generated macro for impl_19 (impl)
macro_rules! Depcrate_formatimpl_19 {
() => {
// Module: crate::format
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > Visit for FmtEvent < 'a > { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { let buf = & mut self . bufs . current_buf ; let comma = if self . comma { "," } else { "" } ; match field . name () { "message" => { write ! (buf , "{} {:?}" , comma , value) . unwrap () ; self . comma = true ; } # [cfg (feature = "tracing-log")] name if name . starts_with ("log.") => { } name => { write ! (buf , "{} {}={:?}" , comma , name , value) . unwrap () ; self . comma = true ; } } } }
};
}
