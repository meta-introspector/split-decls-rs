// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl Tai64 { # [doc = " Unix epoch in `TAI64`: 1970-01-01 00:00:37 TAI."] pub const UNIX_EPOCH : Self = Self (37 + (1 << 62)) ; # [doc = " Length of serialized `TAI64` timestamp in bytes."] pub const BYTE_SIZE : usize = 8 ; # [doc = " Get `TAI64N` timestamp according to system clock."] # [cfg (feature = "std")] pub fn now () -> Self { Tai64N :: now () . into () } # [doc = " Parse `TAI64` from a byte slice"] pub fn from_slice (slice : & [u8]) -> Result < Self , Error > { slice . try_into () } # [doc = " Serialize TAI64 as bytes"] pub fn to_bytes (self) -> [u8 ; Self :: BYTE_SIZE] { self . into () } # [doc = " Convert Unix timestamp to `TAI64`."] pub fn from_unix (secs : i64) -> Self { Tai64 ((secs + 10 + (1 << 62)) as u64) } # [doc = " Convert `TAI64` to unix timestamp."] pub fn to_unix (self) -> i64 { (self . 0 as i64) - (10 + (1 << 62)) } }
};
}
