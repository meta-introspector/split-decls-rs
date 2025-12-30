// Generated macro for impl_446 (impl)
macro_rules! Depcrate_map_response_bodyimpl_446 {
() => {
// Module: crate::map_response_body
// Provides: {"impl_446"}
// Dependencies: {}
impl < Fut , F , ResBody , E , NewResBody > Future for ResponseFuture < Fut , F > where Fut : Future < Output = Result < Response < ResBody > , E > > , F : FnMut (ResBody) -> NewResBody , { type Output = Result < Response < NewResBody > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let res = ready ! (this . inner . poll (cx) ?) ; Poll :: Ready (Ok (res . map (this . f))) } }
};
}
