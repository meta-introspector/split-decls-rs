// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorimpl_20 {
() => {
// Module: crate::error
// Provides: {"impl_20"}
// Dependencies: {}
impl < F > ErrorSink for F where F : FnMut (ParseError) , { fn report_error (& mut self , error : ParseError) { (self) (error) ; } }
};
}
