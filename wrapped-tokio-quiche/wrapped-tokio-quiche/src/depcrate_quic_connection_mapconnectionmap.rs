// Generated macro for ConnectionMap (struct)
macro_rules! Depcrate_quic_connection_mapConnectionMap {
() => {
// Module: crate::quic::connection::map
// Provides: {"ConnectionMap"}
// Dependencies: {}
# [doc = " A map for QUIC connections."] # [doc = ""] # [doc = " Due to the fact that QUIC connections can be identified by multiple QUIC"] # [doc = " connection IDs, we have to be able to map multiple IDs to the same"] # [doc = " connection."] # [derive (Default)] pub (crate) struct ConnectionMap { quic_id_map : BTreeMap < CidOwned , (QuicheId , mpsc :: Sender < Incoming >) > , conn_map : HashMap < QuicheId , mpsc :: Sender < Incoming > > , }
};
}
