// Generated macro for impl_211 (impl)
macro_rules! Depcrate_hedge_delayimpl_211 {
() => {
// Module: crate::hedge::delay
// Provides: {"impl_211"}
// Dependencies: {}
impl < Request , P , S > Service < Request > for Delay < P , S > where P : Policy < Request > , S : Service < Request > + Clone , S :: Error : Into < crate :: BoxError > , { type Response = S :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < Request , S > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn call (& mut self , request : Request) -> Self :: Future { let delay = self . policy . delay (& request) ; ResponseFuture { service : Some (self . service . clone ()) , state : State :: delaying (tokio :: time :: sleep (delay) , Some (request)) , } } }
};
}
