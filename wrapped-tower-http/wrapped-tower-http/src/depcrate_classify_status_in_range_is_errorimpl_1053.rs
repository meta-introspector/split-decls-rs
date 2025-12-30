// Generated macro for impl_1053 (impl)
macro_rules! Depcrate_classify_status_in_range_is_errorimpl_1053 {
() => {
// Module: crate::classify::status_in_range_is_error
// Provides: {"impl_1053"}
// Dependencies: {}
impl fmt :: Display for StatusInRangeFailureClass { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: StatusCode (code) => write ! (f , "Status code: {}" , code) , Self :: Error (error) => write ! (f , "Error: {}" , error) , } } }
};
}
