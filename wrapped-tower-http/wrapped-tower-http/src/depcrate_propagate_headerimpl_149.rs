// Generated macro for impl_149 (impl)
macro_rules! Depcrate_propagate_headerimpl_149 {
() => {
// Module: crate::propagate_header
// Provides: {"impl_149"}
// Dependencies: {}
impl < F , ResBody , E > Future for ResponseFuture < F > where F : Future < Output = Result < Response < ResBody > , E > > , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let mut res = ready ! (this . future . poll (cx) ?) ; if let Some ((header , value)) = this . header_and_value . take () { res . headers_mut () . insert (header , value) ; } Poll :: Ready (Ok (res)) } }
};
}
