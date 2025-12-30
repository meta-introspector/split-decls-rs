// Generated macro for impl_980 (impl)
macro_rules! Depcrate_timeout_serviceimpl_980 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_980"}
// Dependencies: {}
impl < F , B , E > Future for ResponseFuture < F > where F : Future < Output = Result < Response < B > , E > > , B : Default , { type Output = Result < Response < B > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if this . sleep . poll (cx) . is_ready () { let mut res = Response :: new (B :: default ()) ; * res . status_mut () = * this . status_code ; return Poll :: Ready (Ok (res)) ; } this . inner . poll (cx) } }
};
}
