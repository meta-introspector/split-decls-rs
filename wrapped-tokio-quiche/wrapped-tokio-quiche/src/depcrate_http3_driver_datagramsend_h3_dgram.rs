// Generated macro for send_h3_dgram (function)
macro_rules! Depcrate_http3_driver_datagramsend_h3_dgram {
() => {
// Module: crate::http3::driver::datagram
// Provides: {"send_h3_dgram"}
// Dependencies: {}
# [doc = " Sends an HTTP/3 datagram over the QUIC connection with the given `flow_id`."] pub (crate) fn send_h3_dgram (conn : & mut QuicheConnection , flow_id : u64 , mut dgram : PooledDgram ,) -> quiche :: Result < () > { let mut prefix = [0u8 ; 8] ; let mut buf = octets :: OctetsMut :: with_slice (& mut prefix) ; let flow_id = buf . put_varint (flow_id) ? ; if dgram . add_prefix (flow_id) { conn . dgram_send (& dgram) } else { let mut inner = dgram . into_inner () . into_vec () ; inner . splice (.. 0 , flow_id . iter () . copied ()) ; conn . dgram_send_vec (inner) } }
};
}
