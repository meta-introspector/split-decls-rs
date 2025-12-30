// Generated macro for Quic (struct)
macro_rules! Depcrate_quicQuic {
() => {
// Module: crate::quic
// Provides: {"Quic"}
// Dependencies: {}
# [derive (Default)] pub (crate) struct Quic { # [doc = " QUIC transport parameters received from the peer during the handshake"] pub (crate) params : Option < Vec < u8 > > , pub (crate) alert : Option < AlertDescription > , pub (crate) hs_queue : VecDeque < (bool , Vec < u8 >) > , pub (crate) early_secret : Option < OkmBlock > , pub (crate) hs_secrets : Option < Secrets > , pub (crate) traffic_secrets : Option < Secrets > , # [doc = " Whether keys derived from traffic_secrets have been passed to the QUIC implementation"] # [cfg (feature = "std")] pub (crate) returned_traffic_keys : bool , pub (crate) version : Version , }
};
}
