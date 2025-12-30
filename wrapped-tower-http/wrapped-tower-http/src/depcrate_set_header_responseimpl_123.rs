// Generated macro for impl_123 (impl)
macro_rules! Depcrate_set_header_responseimpl_123 {
() => {
// Module: crate::set_header::response
// Provides: {"impl_123"}
// Dependencies: {}
impl < F , ResBody , E , M > Future for ResponseFuture < F , M > where F : Future < Output = Result < Response < ResBody > , E > > , M : MakeHeaderValue < Response < ResBody > > , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let mut res = ready ! (this . future . poll (cx) ?) ; this . mode . apply (this . header_name , & mut res , & mut * this . make) ; Poll :: Ready (Ok (res)) } }
};
}
