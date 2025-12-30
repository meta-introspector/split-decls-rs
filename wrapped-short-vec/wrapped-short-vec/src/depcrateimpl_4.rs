// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl Serialize for ShortU16 { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_tuple (1) ? ; let mut rem_val = self . 0 ; loop { let mut elem = (rem_val & 0x7f) as u8 ; rem_val >>= 7 ; if rem_val == 0 { seq . serialize_element (& elem) ? ; break ; } else { elem |= 0x80 ; seq . serialize_element (& elem) ? ; } } seq . end () } }
};
}
