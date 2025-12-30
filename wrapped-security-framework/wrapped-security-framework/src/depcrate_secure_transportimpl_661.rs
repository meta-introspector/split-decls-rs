// Generated macro for impl_661 (impl)
macro_rules! Depcrate_secure_transportimpl_661 {
() => {
// Module: crate::secure_transport
// Provides: {"impl_661"}
// Dependencies: {}
impl < S : Read + Write > Read for SslStream < S > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } let buffered = self . context () . buffered_read_size () . unwrap_or (0) ; let to_read = if buffered > 0 { cmp :: min (buffered , buf . len ()) } else { buf . len () } ; unsafe { let mut nread = 0 ; let ret = SSLRead (self . ctx . 0 , buf . as_mut_ptr () . cast () , to_read , & mut nread) ; if nread > 0 { return Ok (nread) ; } match ret { errSSLClosedGraceful | errSSLClosedAbort | errSSLClosedNoNotify => Ok (0) , errSSLPeerAuthCompleted => self . read (buf) , _ => Err (self . get_error (ret)) , } } } }
};
}
