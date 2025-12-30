// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Visit for EventVisitor < '_ > { fn record_str (& mut self , field : & Field , value : & str) { self . put_prefix (field) ; put_field_length_encoded (self . buf , field . name () , | buf | { buf . extend_from_slice (value . as_bytes ()) }) ; } fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . put_prefix (field) ; put_field_length_encoded (self . buf , field . name () , | buf | { write ! (buf , "{:?}" , value) . unwrap () }) ; } }
};
}
