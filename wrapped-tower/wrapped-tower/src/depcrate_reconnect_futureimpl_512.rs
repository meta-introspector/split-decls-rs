// Generated macro for impl_512 (impl)
macro_rules! Depcrate_reconnect_futureimpl_512 {
() => {
// Module: crate::reconnect::future
// Provides: {"impl_512"}
// Dependencies: {}
impl < F , T , E , ME > Future for ResponseFuture < F , ME > where F : Future < Output = Result < T , E > > , E : Into < crate :: BoxError > , ME : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let me = self . project () ; match me . inner . project () { InnerProj :: Future { fut } => fut . poll (cx) . map_err (Into :: into) , InnerProj :: Error { error } => { let e = error . take () . expect ("Polled after ready.") . into () ; Poll :: Ready (Err (e)) } } } }
};
}
