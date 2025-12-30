// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl Visit for SpanVisitor < '_ > { fn record_str (& mut self , field : & Field , value : & str) { self . put_span_prefix () ; put_field_length_encoded (self . buf , field . name () , | buf | { buf . extend_from_slice (value . as_bytes ()) }) ; } fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . put_span_prefix () ; put_field_length_encoded (self . buf , field . name () , | buf | { write ! (buf , "{:?}" , value) . unwrap () }) ; } }
};
}
