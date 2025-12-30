// Generated macro for impl_341 (impl)
macro_rules! Depcrate_filters_corsimpl_341 {
() => {
// Module: crate::filters::cors
// Provides: {"impl_341"}
// Dependencies: {}
impl fmt :: Display for CorsForbidden { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let detail = match self . kind { Forbidden :: OriginNotAllowed => "origin not allowed" , Forbidden :: MethodNotAllowed => "request-method not allowed" , Forbidden :: HeaderNotAllowed => "header not allowed" , } ; write ! (f , "CORS request forbidden: {}" , detail) } }
};
}
