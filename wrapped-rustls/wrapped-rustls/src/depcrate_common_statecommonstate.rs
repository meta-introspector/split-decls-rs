// Generated macro for CommonState (struct)
macro_rules! Depcrate_common_stateCommonState {
() => {
// Module: crate::common_state
// Provides: {"CommonState"}
// Dependencies: {}
# [doc = " Connection state common to both client and server connections."] pub struct CommonState { pub (crate) negotiated_version : Option < ProtocolVersion > , pub (crate) handshake_kind : Option < HandshakeKind > , pub (crate) side : Side , pub (crate) record_layer : record_layer :: RecordLayer , pub (crate) suite : Option < SupportedCipherSuite > , pub (crate) kx_state : KxState , pub (crate) alpn_protocol : Option < ProtocolName > , pub (crate) exporter : Option < Box < dyn Exporter > > , pub (crate) early_exporter : Option < Box < dyn Exporter > > , pub (crate) aligned_handshake : bool , pub (crate) may_send_application_data : bool , may_receive_application_data : bool , pub (crate) early_traffic : bool , sent_fatal_alert : bool , # [doc = " If we signaled end of stream."] pub (crate) has_sent_close_notify : bool , # [doc = " If the peer has signaled end of stream."] pub (crate) has_received_close_notify : bool , # [cfg (feature = "std")] pub (crate) has_seen_eof : bool , pub (crate) peer_identity : Option < Identity < 'static > > , message_fragmenter : MessageFragmenter , pub (crate) received_plaintext : ChunkVecBuffer , pub (crate) sendable_tls : ChunkVecBuffer , queued_key_update_message : Option < Vec < u8 > > , # [doc = " Protocol whose key schedule should be used. Unused for TLS < 1.3."] pub (crate) protocol : Protocol , pub (crate) quic : quic :: Quic , pub (crate) enable_secret_extraction : bool , temper_counters : TemperCounters , pub (crate) refresh_traffic_keys_pending : bool , pub (crate) fips : bool , pub (crate) tls13_tickets_received : u32 , }
};
}
