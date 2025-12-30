// Generated macro for impl_31 (impl)
macro_rules! Depcrate_astimpl_31 {
() => {
// Module: crate::ast
// Provides: {"impl_31"}
// Dependencies: {}
impl KeyValue { # [doc = " Creates a new array [key-value element](KeyValue)."] pub (crate) fn new (key : Vec < String > , value : impl Into < Value >) -> Self { Self { key , value : Box :: new (value . into ()) , } } }
};
}
