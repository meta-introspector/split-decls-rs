// Generated macro for unix_imp (module)
macro_rules! Depcrate_netunix_imp {
() => {
// Module: crate::net
// Provides: {"unix_imp"}
// Dependencies: {}
# [cfg (unix)] mod unix_imp { use futures :: TryFutureExt ; use super :: * ; impl Acceptor for tokio :: net :: UnixListener { type Socket = tokio :: net :: UnixStream ; # [inline] fn accept (& self) -> impl Future < Output = tokio :: io :: Result < Self :: Socket > > + Send { tokio :: net :: UnixListener :: accept (self) . and_then (| (s , _) | futures :: future :: ok (s)) } # [inline] fn local_addr (& self) -> tokio :: io :: Result < Option < SocketAddr > > { let addr = tokio :: net :: UnixListener :: local_addr (self) ? ; if let Some (p) = addr . as_pathname () { return Ok (Some (SocketAddr :: Unix (p . to_path_buf ()))) ; } Ok (None) } } impl Connection for std :: os :: unix :: net :: UnixStream { # [inline] fn try_clone (& self) -> std :: io :: Result < Box < dyn Connection > > { let stream = std :: os :: unix :: net :: UnixStream :: try_clone (self) ? ; Ok (Box :: new (stream)) } } }
};
}
