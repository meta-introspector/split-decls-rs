// Generated macro for NewConnection (struct)
macro_rules! Depcrate_quic_routerNewConnection {
() => {
// Module: crate::quic::router
// Provides: {"NewConnection"}
// Dependencies: {}
# [doc = " A [`NewConnection`] describes a new [`quiche::Connection`] that can be"] # [doc = " driven by an io worker."] pub struct NewConnection { conn : QuicheConnection , pending_cid : Option < ConnectionId < 'static > > , initial_pkt : Option < Incoming > , # [doc = " When the handshake started. Should be called before [`quiche::accept`]"] # [doc = " or [`quiche::connect`]."] handshake_start_time : Instant , }
};
}
