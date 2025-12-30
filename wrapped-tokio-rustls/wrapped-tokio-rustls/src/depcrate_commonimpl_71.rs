// Generated macro for impl_71 (impl)
macro_rules! Depcrate_commonimpl_71 {
() => {
// Module: crate::common
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : AsyncRead + Unpin > Read for SyncReadAdapter < '_ , '_ , T > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = ReadBuf :: new (buf) ; match Pin :: new (& mut self . io) . poll_read (self . cx , & mut buf) { Poll :: Ready (Ok (())) => Ok (buf . filled () . len ()) , Poll :: Ready (Err (err)) => Err (err) , Poll :: Pending => Err (io :: ErrorKind :: WouldBlock . into ()) , } } }
};
}
