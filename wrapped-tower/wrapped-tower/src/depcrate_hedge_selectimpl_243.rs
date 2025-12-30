// Generated macro for impl_243 (impl)
macro_rules! Depcrate_hedge_selectimpl_243 {
() => {
// Module: crate::hedge::select
// Provides: {"impl_243"}
// Dependencies: {}
impl < AF , BF , T , AE , BE > Future for ResponseFuture < AF , BF > where AF : Future < Output = Result < T , AE > > , AE : Into < crate :: BoxError > , BF : Future < Output = Result < T , BE > > , BE : Into < crate :: BoxError > , { type Output = Result < T , crate :: BoxError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (r) = this . a_fut . poll (cx) { return Poll :: Ready (Ok (r . map_err (Into :: into) ?)) ; } if let Some (b_fut) = this . b_fut . as_pin_mut () { if let Poll :: Ready (r) = b_fut . poll (cx) { return Poll :: Ready (Ok (r . map_err (Into :: into) ?)) ; } } Poll :: Pending } }
};
}
