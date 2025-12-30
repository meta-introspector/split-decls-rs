// Generated macro for impl_396 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_396 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_396"}
// Dependencies: {}
impl Codec < '_ > for EchConfigExtension { fn encode (& self , bytes : & mut Vec < u8 >) { self . ext_type () . encode (bytes) ; let nested = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; match self { Self :: Unknown (r) => r . encode (nested . buf) , } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { let typ = ExtensionType :: read (r) ? ; let len = u16 :: read (r) ? as usize ; let mut sub = r . sub (len) ? ; # [expect (clippy :: match_single_binding)] let ext = match typ { _ => Self :: Unknown (UnknownExtension :: read (typ , & mut sub)) , } ; sub . expect_empty ("EchConfigExtension") . map (| _ | ext) } }
};
}
