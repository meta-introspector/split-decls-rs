// Generated macro for impl_905 (impl)
macro_rules! Depcrate_request_idimpl_905 {
() => {
// Module: crate::request_id
// Provides: {"impl_905"}
// Dependencies: {}
impl < F , B , E > Future for PropagateRequestIdResponseFuture < F > where F : Future < Output = Result < Response < B > , E > > , { type Output = Result < Response < B > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let mut response = ready ! (this . inner . poll (cx)) ? ; if let Some (current_id) = response . headers () . get (& * this . header_name) { if response . extensions () . get :: < RequestId > () . is_none () { let current_id = current_id . clone () ; response . extensions_mut () . insert (RequestId :: new (current_id)) ; } } else if let Some (request_id) = this . request_id . take () { response . headers_mut () . insert (this . header_name . clone () , request_id . 0 . clone ()) ; response . extensions_mut () . insert (request_id) ; } Poll :: Ready (Ok (response)) } }
};
}
