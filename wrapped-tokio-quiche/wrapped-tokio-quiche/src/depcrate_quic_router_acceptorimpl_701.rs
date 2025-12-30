// Generated macro for impl_701 (impl)
macro_rules! Depcrate_quic_router_acceptorimpl_701 {
() => {
// Module: crate::quic::router::acceptor
// Provides: {"impl_701"}
// Dependencies: {}
impl < S , M > InitialPacketHandler for ConnectionAcceptor < S , M > where S : DatagramSocketSend + Send + 'static , M : Metrics , { fn handle_initials (& mut self , incoming : Incoming , hdr : quiche :: Header < 'static > , quiche_config : & mut quiche :: Config ,) -> io :: Result < Option < NewConnection > > { if hdr . ty != PacketType :: Initial { if let Err (e) = self . cid_generator . verify_connection_id (self . socket_cookie , & hdr . dcid) { self . metrics . invalid_cid_packet_count (e) . inc () ; } Err (labels :: QuicInvalidInitialPacketError :: WrongType (hdr . ty)) ? ; } if ! quiche :: version_is_supported (hdr . version) { return self . handshake_reply (incoming , | buf | { quiche :: negotiate_version (& hdr . scid , & hdr . dcid , buf) . into_io () }) ; } let (scid , original_dcid , pending_cid) = if self . config . disable_client_ip_validation { (self . new_connection_id () , None , Some (hdr . dcid)) } else { let token = hdr . token . as_ref () . unwrap () ; if token . is_empty () { return self . stateless_retry (incoming , hdr) ; } (hdr . dcid , Some (self . token_manager . validate_and_extract_original_dcid (token , incoming . peer_addr) . or (Err (labels :: QuicInvalidInitialPacketError :: TokenValidationFail ,)) ? ,) , None ,) } ; self . accept_conn (incoming , scid , original_dcid . as_ref () , pending_cid , quiche_config ,) } }
};
}
