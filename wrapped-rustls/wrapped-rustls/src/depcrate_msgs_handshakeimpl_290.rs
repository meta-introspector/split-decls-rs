// Generated macro for impl_290 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_290 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'a > Codec < 'a > for ServerExtensions < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let extensions = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; for ext in Self :: ALL_EXTENSIONS { self . encode_one (* ext , extensions . buf) ; } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut out = Self :: default () ; let mut checker = DuplicateExtensionChecker :: new () ; let len = usize :: from (u16 :: read (r) ?) ; let mut sub = r . sub (len) ? ; while sub . any_left () { out . read_one (& mut sub , | unknown | checker . check (unknown)) ? ; } out . unknown_extensions = checker . 0 ; Ok (out) } }
};
}
