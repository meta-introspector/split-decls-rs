// Generated macro for impl_35 (impl)
macro_rules! Depcrate_clientimpl_35 {
() => {
// Module: crate::client
// Provides: {"impl_35"}
// Dependencies: {}
impl < IO > IoSession for TlsStream < IO > { type Io = IO ; type Session = ClientConnection ; # [inline] fn skip_handshake (& self) -> bool { self . state . is_early_data () } # [inline] fn get_mut (& mut self) -> (& mut TlsState , & mut Self :: Io , & mut Self :: Session , & mut bool) { (& mut self . state , & mut self . io , & mut self . session , & mut self . need_flush ,) } # [inline] fn into_io (self) -> Self :: Io { self . io } }
};
}
