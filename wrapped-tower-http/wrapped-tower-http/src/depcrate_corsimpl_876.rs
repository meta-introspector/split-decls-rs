// Generated macro for impl_876 (impl)
macro_rules! Depcrate_corsimpl_876 {
() => {
// Module: crate::cors
// Provides: {"impl_876"}
// Dependencies: {}
impl < F , B , E > Future for ResponseFuture < F > where F : Future < Output = Result < Response < B > , E > > , B : Default , { type Output = Result < Response < B > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . inner . project () { KindProj :: CorsCall { allow_origin_future , allow_origin_complete , future , headers , } => { if ! * allow_origin_complete { headers . extend (ready ! (allow_origin_future . poll (cx))) ; * allow_origin_complete = true ; } let mut response : Response < B > = ready ! (future . poll (cx)) ? ; let response_headers = response . headers_mut () ; if let Some (vary) = headers . remove (header :: VARY) { response_headers . append (header :: VARY , vary) ; } response_headers . extend (headers . drain ()) ; Poll :: Ready (Ok (response)) } KindProj :: PreflightCall { allow_origin_future , headers , } => { headers . extend (ready ! (allow_origin_future . poll (cx))) ; let mut response = Response :: new (B :: default ()) ; mem :: swap (response . headers_mut () , headers) ; Poll :: Ready (Ok (response)) } } } }
};
}
