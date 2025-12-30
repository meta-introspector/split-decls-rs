// Generated macro for ConnectionAcceptor (struct)
macro_rules! Depcrate_quic_router_acceptorConnectionAcceptor {
() => {
// Module: crate::quic::router::acceptor
// Provides: {"ConnectionAcceptor"}
// Dependencies: {}
# [doc = " A [`ConnectionAcceptor`] is an [`InitialPacketHandler`] that acts as a"] # [doc = " server and accepts quic connections."] pub (crate) struct ConnectionAcceptor < S , M > { config : ConnectionAcceptorConfig , socket : Arc < S > , socket_cookie : u64 , token_manager : AddrValidationTokenManager , cid_generator : Box < dyn ConnectionIdGenerator < 'static > > , metrics : M , }
};
}
