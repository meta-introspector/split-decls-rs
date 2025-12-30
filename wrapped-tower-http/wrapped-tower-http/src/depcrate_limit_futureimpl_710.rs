// Generated macro for impl_710 (impl)
macro_rules! Depcrate_limit_futureimpl_710 {
() => {
// Module: crate::limit::future
// Provides: {"impl_710"}
// Dependencies: {}
impl < ResBody , F , E > Future for ResponseFuture < F > where ResBody : Body , F : Future < Output = Result < Response < ResBody > , E > > , { type Output = Result < Response < ResponseBody < ResBody > > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let res = match self . project () . inner . project () { ResFutProj :: PayloadTooLarge => create_error_response () , ResFutProj :: Future { future } => ready ! (future . poll (cx)) ? . map (ResponseBody :: new) , } ; Poll :: Ready (Ok (res)) } }
};
}
