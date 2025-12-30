// Generated macro for test (module)
macro_rules! Depcrate_sockaddrtest {
() => {
// Module: crate::sockaddr
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn inet () { let raw = "127.0.0.1:80" . parse :: < SocketAddrV4 > () . unwrap () ; let addr = SockAddr :: from (raw) ; assert ! (addr . as_inet6 () . is_none ()) ; let addr = addr . as_inet () . unwrap () ; assert_eq ! (raw , addr) ; } # [test] fn inet6 () { let raw = "[2001:db8::ff00:42:8329]:80" . parse :: < SocketAddrV6 > () . unwrap () ; let addr = SockAddr :: from (raw) ; assert ! (addr . as_inet () . is_none ()) ; let addr = addr . as_inet6 () . unwrap () ; assert_eq ! (raw , addr) ; } }
};
}
