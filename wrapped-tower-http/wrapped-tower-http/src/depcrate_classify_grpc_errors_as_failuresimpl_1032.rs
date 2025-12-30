// Generated macro for impl_1032 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1032 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1032"}
// Dependencies: {}
impl fmt :: Display for GrpcFailureClass { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: Code (code) => write ! (f , "Code: {}" , code) , Self :: Error (error) => write ! (f , "Error: {}" , error) , } } }
};
}
