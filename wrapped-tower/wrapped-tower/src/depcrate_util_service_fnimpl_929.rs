// Generated macro for impl_929 (impl)
macro_rules! Depcrate_util_service_fnimpl_929 {
() => {
// Module: crate::util::service_fn
// Provides: {"impl_929"}
// Dependencies: {}
impl < T , F , Request , R , E > Service < Request > for ServiceFn < T > where T : FnMut (Request) -> F , F : Future < Output = Result < R , E > > , { type Response = R ; type Error = E ; type Future = F ; fn poll_ready (& mut self , _ : & mut Context < '_ >) -> Poll < Result < () , E > > { Ok (()) . into () } fn call (& mut self , req : Request) -> Self :: Future { (self . f) (req) } }
};
}
