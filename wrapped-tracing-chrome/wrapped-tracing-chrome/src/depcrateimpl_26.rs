// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > tracing_subscriber :: field :: Visit for JsonVisitor < 'a > { fn record_debug (& mut self , field : & Field , value : & dyn std :: fmt :: Debug) { self . object . insert (field . name () . to_owned () , format ! ("{value:?}") . into ()) ; } }
};
}
