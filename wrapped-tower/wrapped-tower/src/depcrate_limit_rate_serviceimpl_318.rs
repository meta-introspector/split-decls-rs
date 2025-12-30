// Generated macro for impl_318 (impl)
macro_rules! Depcrate_limit_rate_serviceimpl_318 {
() => {
// Module: crate::limit::rate::service
// Provides: {"impl_318"}
// Dependencies: {}
impl < S , Request > Service < Request > for RateLimit < S > where S : Service < Request > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . state { State :: Ready { .. } => return Poll :: Ready (ready ! (self . inner . poll_ready (cx))) , State :: Limited => { if Pin :: new (& mut self . sleep) . poll (cx) . is_pending () { tracing :: trace ! ("rate limit exceeded; sleeping.") ; return Poll :: Pending ; } } } self . state = State :: Ready { until : Instant :: now () + self . rate . per () , rem : self . rate . num () , } ; Poll :: Ready (ready ! (self . inner . poll_ready (cx))) } fn call (& mut self , request : Request) -> Self :: Future { match self . state { State :: Ready { mut until , mut rem } => { let now = Instant :: now () ; if now >= until { until = now + self . rate . per () ; rem = self . rate . num () ; } if rem > 1 { rem -= 1 ; self . state = State :: Ready { until , rem } ; } else { self . sleep . as_mut () . reset (until) ; self . state = State :: Limited ; } self . inner . call (request) } State :: Limited => panic ! ("service not ready; poll_ready must be called first") , } } }
};
}
