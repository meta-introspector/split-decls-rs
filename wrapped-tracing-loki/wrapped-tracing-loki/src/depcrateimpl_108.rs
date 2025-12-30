// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
impl Fields { fn record_impl (& mut self , field : & Field , value : serde_json :: Value) { self . fields . insert (field . name () . into () , value) ; } fn record < T : Into < serde_json :: Value > > (& mut self , field : & Field , value : T) { self . record_impl (field , value . into ()) ; } }
};
}
