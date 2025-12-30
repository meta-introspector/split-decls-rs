// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_serverimpl_1437 {
() => {
// Module: crate::server
// Provides: {"impl_1437"}
// Dependencies: {}
impl Future for ShutdownOrInactive { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { loop { match Pin :: new (& mut self . rx) . poll_next (cx) { Poll :: Pending => break , Poll :: Ready (Some (ServerMessage :: Shutdown)) => return Poll :: Ready (()) , Poll :: Ready (Some (ServerMessage :: Request)) => { if self . timeout_dur != Duration :: new (0 , 0) { self . timeout = Some (Box :: pin (sleep (self . timeout_dur))) ; } } Poll :: Ready (None) => return Poll :: Ready (()) , } } match self . timeout { None => Poll :: Pending , Some (ref mut timeout) => timeout . as_mut () . poll (cx) , } } }
};
}
