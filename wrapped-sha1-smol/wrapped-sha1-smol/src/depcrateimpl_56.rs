// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl str :: FromStr for Digest { type Err = DigestParseError ; fn from_str (s : & str) -> Result < Digest , DigestParseError > { if s . len () != 40 { return Err (DigestParseError (())) ; } let mut rv : Digest = Default :: default () ; for idx in 0 .. 5 { rv . data . state [idx] = r#try ! (u32 :: from_str_radix (& s [idx * 8 .. idx * 8 + 8] , 16) . map_err (| _ | DigestParseError (()))) ; } Ok (rv) } }
};
}
