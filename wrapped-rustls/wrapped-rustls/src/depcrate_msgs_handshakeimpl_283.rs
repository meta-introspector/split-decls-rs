// Generated macro for impl_283 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_283 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a > Codec < 'a > for ClientExtensions < 'a > { fn encode (& self , bytes : & mut Vec < u8 >) { let order = self . used_extensions_in_encoding_order () ; if order . is_empty () { return ; } let body = LengthPrefixedBuffer :: new (ListLength :: U16 , bytes) ; for item in order { self . encode_one (item , body . buf) ; } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { let mut out = Self :: default () ; if ! r . any_left () { return Ok (out) ; } let mut checker = DuplicateExtensionChecker :: new () ; let len = usize :: from (u16 :: read (r) ?) ; let mut sub = r . sub (len) ? ; while sub . any_left () { let typ = out . read_one (& mut sub , | unknown | checker . check (unknown)) ? ; if typ == ExtensionType :: PreSharedKey && sub . any_left () { return Err (InvalidMessage :: PreSharedKeyIsNotFinalExtension) ; } } Ok (out) } }
};
}
