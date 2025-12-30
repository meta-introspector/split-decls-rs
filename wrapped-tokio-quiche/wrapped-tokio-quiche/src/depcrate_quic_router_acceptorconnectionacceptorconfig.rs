// Generated macro for ConnectionAcceptorConfig (struct)
macro_rules! Depcrate_quic_router_acceptorConnectionAcceptorConfig {
() => {
// Module: crate::quic::router::acceptor
// Provides: {"ConnectionAcceptorConfig"}
// Dependencies: {}
pub (crate) struct ConnectionAcceptorConfig { pub (crate) disable_client_ip_validation : bool , pub (crate) qlog_dir : Option < String > , pub (crate) keylog_file : Option < File > , # [cfg (target_os = "linux")] pub (crate) with_pktinfo : bool , }
};
}
