// Generated macro for impl_1275 (impl)
macro_rules! Depcrate_validate_requestimpl_1275 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1275"}
// Dependencies: {}
impl < F , B , E > Future for ResponseFuture < F , B > where F : Future < Output = Result < Response < B > , E > > , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . kind . project () { KindProj :: Future { future } => future . poll (cx) , KindProj :: Error { response } => { let response = response . take () . expect ("future polled after completion") ; Poll :: Ready (Ok (response)) } } } }
};
}
