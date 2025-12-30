// Generated macro for compare_ignore_zoneid (function)
macro_rules! Depcrate_net_testcompare_ignore_zoneid {
() => {
// Module: crate::net::test
// Provides: {"compare_ignore_zoneid"}
// Dependencies: {}
pub fn compare_ignore_zoneid (a : & SocketAddr , b : & SocketAddr) -> bool { match (a , b) { (SocketAddr :: V6 (a) , SocketAddr :: V6 (b)) => { a . ip () . segments () == b . ip () . segments () && a . flowinfo () == b . flowinfo () && a . port () == b . port () } _ => a == b , } }
};
}
