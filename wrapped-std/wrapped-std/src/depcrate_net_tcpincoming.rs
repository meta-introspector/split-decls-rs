// Generated macro for Incoming (struct)
macro_rules! Depcrate_net_tcpIncoming {
() => {
// Module: crate::net::tcp
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " An iterator that infinitely [`accept`]s connections on a [`TcpListener`]."] # [doc = ""] # [doc = " This `struct` is created by the [`TcpListener::incoming`] method."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`accept`]: TcpListener::accept"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Incoming < 'a > { listener : & 'a TcpListener , }
};
}
