// Generated macro for macro_85 (macro)
macro_rules! Depcrate_buffer_futuremacro_85 {
() => {
// Module: crate::buffer::future
// Provides: {"macro_85"}
// Dependencies: {}
pin_project ! { # [project = ResponseStateProj] # [derive (Debug)] enum ResponseState < T > { Failed { error : Option < crate :: BoxError >, } , Rx { # [pin] rx : message :: Rx < T >, } , Poll { # [pin] fut : T , } , } }
};
}
