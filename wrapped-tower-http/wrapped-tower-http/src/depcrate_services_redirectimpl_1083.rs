// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_services_redirectimpl_1083 {
() => {
// Module: crate::services::redirect
// Provides: {"impl_1083"}
// Dependencies: {}
impl < ResBody > Future for ResponseFuture < ResBody > where ResBody : Default , { type Output = Result < Response < ResBody > , Infallible > ; fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut res = Response :: default () ; * res . status_mut () = self . status_code ; res . headers_mut () . insert (header :: LOCATION , self . location . take () . unwrap ()) ; Poll :: Ready (Ok (res)) } }
};
}
