// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < S > Read for AllowStd < S > where S : AsyncRead + Unpin , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let mut buf = ReadBuf :: new (buf) ; self . with_context (| ctx , stream | stream . poll_read (ctx , & mut buf)) ? ; Ok (buf . filled () . len ()) } }
};
}
