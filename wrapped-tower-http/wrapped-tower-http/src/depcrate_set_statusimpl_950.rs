// Generated macro for impl_950 (impl)
macro_rules! Depcrate_set_statusimpl_950 {
() => {
// Module: crate::set_status
// Provides: {"impl_950"}
// Dependencies: {}
impl < F , B , E > Future for ResponseFuture < F > where F : Future < Output = Result < Response < B > , E > > , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let mut response = ready ! (this . inner . poll (cx) ?) ; * response . status_mut () = this . status . take () . expect ("future polled after completion") ; Poll :: Ready (Ok (response)) } }
};
}
