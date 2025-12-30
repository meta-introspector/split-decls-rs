// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl Visit for Data { fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { self . kvs . push ((field . name () , format ! ("{:?}" , value))) } }
};
}
