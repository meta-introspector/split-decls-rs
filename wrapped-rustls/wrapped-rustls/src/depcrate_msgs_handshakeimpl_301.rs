// Generated macro for impl_301 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_301 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_301"}
// Dependencies: {}
impl < 'a > Codec < 'a > for HelloRetryRequestExtensions < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let extensions = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; for ext in self . order . as_deref () . unwrap_or (Self :: ALL_EXTENSIONS) { self . encode_one (* ext , extensions . buf) ; } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut out = Self :: default () ; let mut order = vec ! [] ; let len = usize :: from (u16 :: read (r) ?) ; let mut sub = r . sub (len) ? ; while sub . any_left () { let typ = out . read_one (& mut sub , | _unk | { Err (InvalidMessage :: UnknownHelloRetryRequestExtension) }) ? ; order . push (typ) ; } out . order = Some (order) ; Ok (out) } }
};
}
