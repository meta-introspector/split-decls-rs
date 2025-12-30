// Generated macro for impl_447 (impl)
macro_rules! Depcrate_quic_connection_idimpl_447 {
() => {
// Module: crate::quic::connection::id
// Provides: {"impl_447"}
// Dependencies: {}
impl ConnectionIdGenerator < 'static > for SimpleConnectionIdGenerator { fn new_connection_id (& self , _socket_cookie : u64) -> ConnectionId < 'static > { let mut buf = vec ! [0 ; 20] ; boring :: rand :: rand_bytes (& mut buf) . unwrap () ; ConnectionId :: from_vec (buf) } # [doc = " Performs no verification, because this generator can create"] # [doc = " any valid connection ID."] fn verify_connection_id (& self , _socket_cookie : u64 , _cid : & ConnectionId < '_ > ,) -> QuicResult < () > { Ok (()) } }
};
}
