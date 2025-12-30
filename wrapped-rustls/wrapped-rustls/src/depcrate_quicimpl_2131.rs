// Generated macro for impl_2131 (impl)
macro_rules! Depcrate_quicimpl_2131 {
() => {
// Module: crate::quic
// Provides: {"impl_2131"}
// Dependencies: {}
impl PacketKeySet { fn new (secrets : & Secrets) -> Self { let (local , remote) = secrets . local_remote () ; let (version , alg , hkdf) = (secrets . version , secrets . quic , secrets . suite . hkdf_provider) ; Self { local : KeyBuilder :: new (local , version , alg , hkdf) . packet_key () , remote : KeyBuilder :: new (remote , version , alg , hkdf) . packet_key () , } } }
};
}
