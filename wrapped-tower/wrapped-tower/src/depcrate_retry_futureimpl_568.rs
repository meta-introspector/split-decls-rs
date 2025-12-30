// Generated macro for impl_568 (impl)
macro_rules! Depcrate_retry_futureimpl_568 {
() => {
// Module: crate::retry::future
// Provides: {"impl_568"}
// Dependencies: {}
impl < P , S , Request > Future for ResponseFuture < P , S , Request > where P : Policy < Request , S :: Response , S :: Error > , S : Service < Request > , { type Output = Result < S :: Response , S :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state . as_mut () . project () { StateProj :: Called { future } => { let mut result = ready ! (future . poll (cx)) ; if let Some (req) = & mut this . request { match this . retry . policy . retry (req , & mut result) { Some (waiting) => { this . state . set (State :: Waiting { waiting }) ; } None => return Poll :: Ready (result) , } } else { return Poll :: Ready (result) ; } } StateProj :: Waiting { waiting } => { ready ! (waiting . poll (cx)) ; this . state . set (State :: Retrying) ; } StateProj :: Retrying => { ready ! (this . retry . as_mut () . project () . service . poll_ready (cx)) ? ; let req = this . request . take () . expect ("retrying requires cloned request") ; * this . request = this . retry . policy . clone_request (& req) ; this . state . set (State :: Called { future : this . retry . as_mut () . project () . service . call (req) , }) ; } } } } }
};
}
