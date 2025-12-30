// Generated macro for receive_h3_dgram (function)
macro_rules! Depcrate_http3_driver_datagramreceive_h3_dgram {
() => {
// Module: crate::http3::driver::datagram
// Provides: {"receive_h3_dgram"}
// Dependencies: {}
# [doc = " Reads the next HTTP/3 datagram from the QUIC connection."] # [doc = ""] # [doc = " [`quiche::Error::Done`] is returned if there is no datagram to read."] pub (crate) fn receive_h3_dgram (conn : & mut QuicheConnection ,) -> quiche :: Result < (u64 , InboundFrame) > { let dgram = conn . dgram_recv_vec () ? ; let mut buf = octets :: Octets :: with_slice (& dgram) ; let flow_id = buf . get_varint () ? ; let advance = buf . off () ; let datagram = InboundFrame :: Datagram (BufFactory :: dgram_from_slice (& dgram [advance ..])) ; Ok ((flow_id , datagram)) }
};
}
