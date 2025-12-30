// Generated macro for impl_871 (impl)
macro_rules! Depcrate_util_map_futureimpl_871 {
() => {
// Module: crate::util::map_future
// Provides: {"impl_871"}
// Dependencies: {}
impl < R , S , F , T , E , Fut > Service < R > for MapFuture < S , F > where S : Service < R > , F : FnMut (S :: Future) -> Fut , E : From < S :: Error > , Fut : Future < Output = Result < T , E > > , { type Response = T ; type Error = E ; type Future = Fut ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (From :: from) } fn call (& mut self , req : R) -> Self :: Future { (self . f) (self . inner . call (req)) } }
};
}
