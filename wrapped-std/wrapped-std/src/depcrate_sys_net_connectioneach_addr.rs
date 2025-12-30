// Generated macro for each_addr (function)
macro_rules! Depcrate_sys_net_connectioneach_addr {
() => {
// Module: crate::sys::net::connection
// Provides: {"each_addr"}
// Dependencies: {}
# [cfg_attr (not (any (target_os = "linux" , target_os = "windows")) , allow (dead_code))] fn each_addr < A : crate :: net :: ToSocketAddrs , F , T > (addr : A , mut f : F) -> crate :: io :: Result < T > where F : FnMut (& crate :: net :: SocketAddr) -> crate :: io :: Result < T > , { use crate :: io :: Error ; let mut last_err = None ; for addr in addr . to_socket_addrs () ? { match f (& addr) { Ok (l) => return Ok (l) , Err (e) => last_err = Some (e) , } } match last_err { Some (err) => Err (err) , None => Err (Error :: NO_ADDRESSES) , } }
};
}
