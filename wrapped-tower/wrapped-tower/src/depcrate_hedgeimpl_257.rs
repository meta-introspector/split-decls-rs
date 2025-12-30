// Generated macro for impl_257 (impl)
macro_rules! Depcrate_hedgeimpl_257 {
() => {
// Module: crate::hedge
// Provides: {"impl_257"}
// Dependencies: {}
impl < S , P , Request > tower_service :: Service < Request > for Hedge < S , P > where S : tower_service :: Service < Request > + Clone , S :: Error : Into < crate :: BoxError > , P : Policy < Request > + Clone , { type Response = S :: Response ; type Error = crate :: BoxError ; type Future = Future < Service < S , P > , Request > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . 0 . poll_ready (cx) } fn call (& mut self , request : Request) -> Self :: Future { Future { inner : self . 0 . call (request) , } } }
};
}
