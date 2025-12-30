// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Tai64 { type Error = Error ; fn try_from (slice : & 'a [u8]) -> Result < Self , Error > { let bytes : [u8 ; Tai64 :: BYTE_SIZE] = slice . try_into () . map_err (| _ | Error :: LengthInvalid) ? ; Ok (bytes . into ()) } }
};
}
