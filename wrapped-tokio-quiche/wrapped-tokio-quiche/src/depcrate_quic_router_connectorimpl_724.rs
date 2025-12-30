// Generated macro for impl_724 (impl)
macro_rules! Depcrate_quic_router_connectorimpl_724 {
() => {
// Module: crate::quic::router::connector
// Provides: {"impl_724"}
// Dependencies: {}
impl ConnectionState { fn take_if_queued (& mut self) -> Option < QuicheConnection > { match mem :: replace (self , Self :: Returned) { Self :: Queued (conn) => Some (conn) , state => { * self = state ; None } , } } fn take_if_pending_and_id_matches (& mut self , scid : & ConnectionId < 'static > ,) -> Option < PendingConnection > { match mem :: replace (self , Self :: Returned) { Self :: Pending (pending) if * scid == pending . conn . source_id () => Some (pending) , state => { * self = state ; None } , } } }
};
}
