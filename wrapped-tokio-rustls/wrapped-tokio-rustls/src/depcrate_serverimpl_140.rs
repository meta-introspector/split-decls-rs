// Generated macro for impl_140 (impl)
macro_rules! Depcrate_serverimpl_140 {
() => {
// Module: crate::server
// Provides: {"impl_140"}
// Dependencies: {}
impl < IO > IoSession for TlsStream < IO > { type Io = IO ; type Session = ServerConnection ; # [inline] fn skip_handshake (& self) -> bool { false } # [inline] fn get_mut (& mut self) -> (& mut TlsState , & mut Self :: Io , & mut Self :: Session , & mut bool) { (& mut self . state , & mut self . io , & mut self . session , & mut self . need_flush ,) } # [inline] fn into_io (self) -> Self :: Io { self . io } }
};
}
