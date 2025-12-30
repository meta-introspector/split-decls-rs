// Generated macro for impl_617 (impl)
macro_rules! Depcrate_steerimpl_617 {
() => {
// Module: crate::steer
// Provides: {"impl_617"}
// Dependencies: {}
impl < S , Req , F > Service < Req > for Steer < S , F , Req > where S : Service < Req > , F : Picker < S , Req > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { loop { if self . not_ready . is_empty () { return Poll :: Ready (Ok (())) ; } else { if self . services [self . not_ready [0]] . poll_ready (cx) ? . is_pending () { return Poll :: Pending ; } self . not_ready . pop_front () ; } } } fn call (& mut self , req : Req) -> Self :: Future { assert ! (self . not_ready . is_empty () , "Steer must wait for all services to be ready. Did you forget to call poll_ready()?") ; let idx = self . router . pick (& req , & self . services [..]) ; let cl = & mut self . services [idx] ; self . not_ready . push_back (idx) ; cl . call (req) } }
};
}
