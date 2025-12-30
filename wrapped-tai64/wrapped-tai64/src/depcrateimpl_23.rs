// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Tai64N { type Error = Error ; fn try_from (slice : & 'a [u8]) -> Result < Self , Error > { let bytes : [u8 ; Tai64N :: BYTE_SIZE] = slice . try_into () . map_err (| _ | Error :: LengthInvalid) ? ; bytes . try_into () } }
};
}
