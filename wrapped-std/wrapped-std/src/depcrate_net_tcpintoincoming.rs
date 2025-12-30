// Generated macro for IntoIncoming (struct)
macro_rules! Depcrate_net_tcpIntoIncoming {
() => {
// Module: crate::net::tcp
// Provides: {"IntoIncoming"}
// Dependencies: {}
# [doc = " An iterator that infinitely [`accept`]s connections on a [`TcpListener`]."] # [doc = ""] # [doc = " This `struct` is created by the [`TcpListener::into_incoming`] method."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`accept`]: TcpListener::accept"] # [derive (Debug)] # [unstable (feature = "tcplistener_into_incoming" , issue = "88373")] pub struct IntoIncoming { listener : TcpListener , }
};
}
