// Generated macro for impl_92 (impl)
macro_rules! Depcrate_timestampimpl_92 {
() => {
// Module: crate::timestamp
// Provides: {"impl_92"}
// Dependencies: {}
# [doc (hidden)] impl Timestamp { # [deprecated (since = "1.10.0" , note = "use `Timestamp::from_gregorian(ticks, counter)`")] pub const fn from_rfc4122 (ticks : u64 , counter : u16) -> Self { Timestamp :: from_gregorian (ticks , counter) } # [deprecated (since = "1.10.0" , note = "use `Timestamp::to_gregorian()`")] pub const fn to_rfc4122 (& self) -> (u64 , u16) { self . to_gregorian () } # [deprecated (since = "1.2.0" , note = "`Timestamp::to_unix_nanos()` is deprecated and will be removed: use `Timestamp::to_unix()`")] pub const fn to_unix_nanos (& self) -> u32 { panic ! ("`Timestamp::to_unix_nanos()` is deprecated and will be removed: use `Timestamp::to_unix()`") } }
};
}
