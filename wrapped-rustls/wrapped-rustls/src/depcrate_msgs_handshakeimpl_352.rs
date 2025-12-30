// Generated macro for impl_352 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_352 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_352"}
// Dependencies: {}
impl ServerKeyExchangePayload { pub (crate) fn unwrap_given_kxa (& self , kxa : KeyExchangeAlgorithm) -> Option < ServerKeyExchange > { if let Self :: Unknown (unk) = self { let mut rd = Reader :: init (unk . bytes ()) ; let result = ServerKeyExchange { params : ServerKeyExchangeParams :: decode (& mut rd , kxa) . ok () ? , dss : DigitallySignedStruct :: read (& mut rd) . ok () ? , } ; if ! rd . any_left () { return Some (result) ; } ; } None } }
};
}
