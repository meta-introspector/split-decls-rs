// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl TryFrom < [u8 ; Self :: BYTE_SIZE] > for Tai64N { type Error = Error ; # [doc = " Parse TAI64 from external representation"] fn try_from (bytes : [u8 ; Tai64N :: BYTE_SIZE]) -> Result < Self , Error > { let secs = Tai64 :: from_slice (& bytes [.. Tai64 :: BYTE_SIZE]) ? ; let mut nano_bytes = [0u8 ; 4] ; nano_bytes . copy_from_slice (& bytes [Tai64 :: BYTE_SIZE ..]) ; let nanos = u32 :: from_be_bytes (nano_bytes) ; if nanos < NANOS_PER_SECOND { Ok (Tai64N (secs , nanos)) } else { Err (Error :: NanosInvalid) } } }
};
}
