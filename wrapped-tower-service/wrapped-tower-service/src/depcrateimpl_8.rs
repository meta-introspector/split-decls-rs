// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < S , Request > Service < Request > for & mut S where S : Service < Request > + ? Sized , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { (* * self) . poll_ready (cx) } fn call (& mut self , request : Request) -> S :: Future { (* * self) . call (request) } }
};
}
