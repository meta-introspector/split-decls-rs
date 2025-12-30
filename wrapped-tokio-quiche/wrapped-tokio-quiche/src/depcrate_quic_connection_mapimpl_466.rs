// Generated macro for impl_466 (impl)
macro_rules! Depcrate_quic_connection_mapimpl_466 {
() => {
// Module: crate::quic::connection::map
// Provides: {"impl_466"}
// Dependencies: {}
impl ConnectionMap { pub (crate) fn insert < Tx , M > (& mut self , cid : ConnectionId < '_ > , conn : & InitialQuicConnection < Tx , M > ,) where Tx : DatagramSocketSend + Send + 'static , M : Metrics , { let id = conn . id ; let ev_sender = conn . incoming_ev_sender . clone () ; self . conn_map . insert (id , ev_sender . clone ()) ; self . quic_id_map . insert ((& cid) . into () , (id , ev_sender)) ; } pub (crate) fn remove (& mut self , cid : & ConnectionId < '_ >) { if let Some ((id , _)) = self . quic_id_map . remove (& cid . into ()) { self . conn_map . remove (& id) ; } } pub (crate) fn map_cid < Tx , M > (& mut self , cid : ConnectionId < '_ > , conn : & InitialQuicConnection < Tx , M > ,) where Tx : DatagramSocketSend + Send + 'static , M : Metrics , { let id = conn . id ; if let Some (ev_sender) = self . conn_map . get (& id) { self . quic_id_map . insert ((& cid) . into () , (id , ev_sender . clone ())) ; } } pub (crate) fn unmap_cid (& mut self , cid : & ConnectionId < '_ >) { self . quic_id_map . remove (& cid . into ()) ; } pub (crate) fn get (& self , id : & ConnectionId ,) -> Option < & mpsc :: Sender < Incoming > > { if id . len () == MAX_CONN_ID_LEN { self . quic_id_map . get (& id . into ()) . map (| (_id , sender) | sender) } else { self . quic_id_map . get (& id . into ()) . map (| (_id , sender) | sender) } } }
};
}
