// Generated macro for impl_190 (impl)
macro_rules! Depcrate_filterimpl_190 {
() => {
// Module: crate::filter
// Provides: {"impl_190"}
// Dependencies: {}
impl < T , U , Request > Service < Request > for AsyncFilter < T , U > where U : AsyncPredicate < Request > , T : Service < U :: Request > + Clone , T :: Error : Into < BoxError > , { type Response = T :: Response ; type Error = BoxError ; type Future = AsyncResponseFuture < U , T , Request > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (Into :: into) } fn call (& mut self , request : Request) -> Self :: Future { use std :: mem ; let inner = self . inner . clone () ; let inner = mem :: replace (& mut self . inner , inner) ; let check = self . predicate . check (request) ; AsyncResponseFuture :: new (check , inner) } }
};
}
