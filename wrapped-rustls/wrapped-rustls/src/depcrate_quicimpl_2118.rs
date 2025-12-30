// Generated macro for impl_2118 (impl)
macro_rules! Depcrate_quicimpl_2118 {
() => {
// Module: crate::quic
// Provides: {"impl_2118"}
// Dependencies: {}
# [cfg (feature = "std")] impl Quic { pub (crate) fn write_hs (& mut self , buf : & mut Vec < u8 >) -> Option < KeyChange > { while let Some ((_ , msg)) = self . hs_queue . pop_front () { buf . extend_from_slice (& msg) ; if let Some (& (true , _)) = self . hs_queue . front () { if self . hs_secrets . is_some () { break ; } } } if let Some (secrets) = self . hs_secrets . take () { return Some (KeyChange :: Handshake { keys : Keys :: new (& secrets) , }) ; } if let Some (mut secrets) = self . traffic_secrets . take () { if ! self . returned_traffic_keys { self . returned_traffic_keys = true ; let keys = Keys :: new (& secrets) ; secrets . update () ; return Some (KeyChange :: OneRtt { keys , next : secrets , }) ; } } None } }
};
}
