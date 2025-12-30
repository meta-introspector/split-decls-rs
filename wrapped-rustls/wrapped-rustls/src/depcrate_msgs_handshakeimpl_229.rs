// Generated macro for impl_229 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_229 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_229"}
// Dependencies: {}
impl Codec < '_ > for SupportedEcPointFormats { fn encode (& self , bytes : & mut Vec < u8 >) { let inner = LengthPrefixedBuffer :: new (ECPointFormat :: SIZE_LEN , bytes) ; if self . uncompressed { ECPointFormat :: Uncompressed . encode (inner . buf) ; } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let mut uncompressed = false ; for pf in TlsListIter :: < ECPointFormat > :: new (r) ? { if let ECPointFormat :: Uncompressed = pf ? { uncompressed = true ; } } Ok (Self { uncompressed }) } }
};
}
