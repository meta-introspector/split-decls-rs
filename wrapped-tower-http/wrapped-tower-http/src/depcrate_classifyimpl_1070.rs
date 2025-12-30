// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_classifyimpl_1070 {
() => {
// Module: crate::classify
// Provides: {"impl_1070"}
// Dependencies: {}
impl fmt :: Display for ServerErrorsFailureClass { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: StatusCode (code) => write ! (f , "Status code: {}" , code) , Self :: Error (error) => write ! (f , "Error: {}" , error) , } } }
};
}
