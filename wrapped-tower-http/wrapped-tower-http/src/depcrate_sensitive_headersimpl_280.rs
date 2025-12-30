// Generated macro for impl_280 (impl)
macro_rules! Depcrate_sensitive_headersimpl_280 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_280"}
// Dependencies: {}
impl < F , ResBody , E > Future for SetSensitiveResponseHeadersResponseFuture < F > where F : Future < Output = Result < Response < ResBody > , E > > , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let mut res = ready ! (this . future . poll (cx) ?) ; let headers = res . headers_mut () ; for header in & * * this . headers { if let http :: header :: Entry :: Occupied (mut entry) = headers . entry (header) { for value in entry . iter_mut () { value . set_sensitive (true) ; } } } Poll :: Ready (Ok (res)) } }
};
}
