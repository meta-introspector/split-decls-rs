// Generated macro for impl_188 (impl)
macro_rules! Depcrate_timeimpl_188 {
() => {
// Module: crate::time
// Provides: {"impl_188"}
// Dependencies: {}
impl Time { # [doc = " Time used for Certificate who do not expire."] pub const INFINITY : Time = Time :: GeneralTime (GeneralizedTime :: from_date_time (DateTime :: INFINITY)) ; # [doc = " Get duration since `UNIX_EPOCH`."] pub fn to_unix_duration (self) -> Duration { match self { Time :: UtcTime (t) => t . to_unix_duration () , Time :: GeneralTime (t) => t . to_unix_duration () , } } # [doc = " Get Time as DateTime"] pub fn to_date_time (& self) -> DateTime { match self { Time :: UtcTime (t) => t . to_date_time () , Time :: GeneralTime (t) => t . to_date_time () , } } # [doc = " Convert to [`SystemTime`]."] # [cfg (feature = "std")] pub fn to_system_time (& self) -> SystemTime { match self { Time :: UtcTime (t) => t . to_system_time () , Time :: GeneralTime (t) => t . to_system_time () , } } # [doc = " Convert time to UTCTime representation"] # [doc = " As per RFC 5280: 4.1.2.5, date through 2049 should be expressed as UTC Time."] pub (crate) fn rfc5280_adjust_utc_time (& mut self) -> der :: Result < () > { if let Time :: GeneralTime (t) = self { let date = t . to_date_time () ; if date . year () <= UtcTime :: MAX_YEAR { * self = Time :: UtcTime (UtcTime :: from_date_time (date) ?) ; } } Ok (()) } # [doc = " Creates a `Time` from the current date."] # [cfg (feature = "std")] pub fn now () -> der :: Result < Self > { SystemTime :: now () . try_into () } }
};
}
