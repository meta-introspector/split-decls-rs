// Generated macro for impl_188 (impl)
macro_rules! Depcrate_filterimpl_188 {
() => {
// Module: crate::filter
// Provides: {"impl_188"}
// Dependencies: {}
impl < T , U , Request > Service < Request > for Filter < T , U > where U : Predicate < Request > , T : Service < U :: Request > , T :: Error : Into < BoxError > , { type Response = T :: Response ; type Error = BoxError ; type Future = ResponseFuture < T :: Response , T :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (Into :: into) } fn call (& mut self , request : Request) -> Self :: Future { ResponseFuture :: new (match self . predicate . check (request) { Ok (request) => Either :: Right (self . inner . call (request) . err_into ()) , Err (e) => Either :: Left (std :: future :: ready (Err (e))) , }) } }
};
}
