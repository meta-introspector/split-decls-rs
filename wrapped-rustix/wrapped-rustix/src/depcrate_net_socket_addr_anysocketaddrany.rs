// Generated macro for SocketAddrAny (struct)
macro_rules! Depcrate_net_socket_addr_anySocketAddrAny {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"SocketAddrAny"}
// Dependencies: {}
# [doc = " A type that can hold any kind of socket address, as a safe abstraction for"] # [doc = " `sockaddr_storage`."] # [doc = ""] # [doc = " Socket addresses can be converted to `SocketAddrAny` via the [`From`] and"] # [doc = " [`Into`] traits. `SocketAddrAny` can be converted back to a specific socket"] # [doc = " address type with [`TryFrom`] and [`TryInto`]. These implementations return"] # [doc = " [`Errno::AFNOSUPPORT`] if the address family does not match the requested"] # [doc = " type."] # [derive (Clone)] # [doc (alias = "sockaddr_storage")] pub struct SocketAddrAny { pub (crate) len : NonZeroU32 , pub (crate) storage : MaybeUninit < SocketAddrStorage > , }
};
}
