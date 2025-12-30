// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl FromStr for Signature { type Err = ParseSignatureError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use five8 :: DecodeError ; if s . len () > MAX_BASE58_SIGNATURE_LEN { return Err (ParseSignatureError :: WrongSize) ; } let mut bytes = [0 ; SIGNATURE_BYTES] ; five8 :: decode_64 (s , & mut bytes) . map_err (| e | match e { DecodeError :: InvalidChar (_) => ParseSignatureError :: Invalid , DecodeError :: TooLong | DecodeError :: TooShort | DecodeError :: LargestTermTooHigh | DecodeError :: OutputTooLong => ParseSignatureError :: WrongSize , }) ? ; Ok (Self :: from (bytes)) } }
};
}
