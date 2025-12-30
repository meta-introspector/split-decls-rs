// Generated macro for SccacheResponse (type)
macro_rules! Depcrate_serverSccacheResponse {
() => {
// Module: crate::server
// Provides: {"SccacheResponse"}
// Dependencies: {}
type SccacheResponse = Message < Response , Pin < Box < dyn Future < Output = Result < Response > > + Send > > > ;
};
}
