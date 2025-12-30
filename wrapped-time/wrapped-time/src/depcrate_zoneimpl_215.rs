// Generated macro for impl_215 (impl)
macro_rules! Depcrate_zoneimpl_215 {
() => {
// Module: crate::zone
// Provides: {"impl_215"}
// Dependencies: {}
impl TimeZoneVariant { # [doc = " Creates a zone variant from a TZDB `isdst` flag, if it is known that the TZDB was built with"] # [doc = " `DATAFORM=rearguard`."] # [doc = ""] # [doc = " If it is known that the database was *not* built with `rearguard`, a caller can try to adjust"] # [doc = " for the differences. This is a moving target, for example the known differences for 2025a are:"] # [doc = ""] # [doc = " * `Europe/Dublin` since 1968-10-27"] # [doc = " * `Africa/Windhoek` between 1994-03-20 and 2017-10-24"] # [doc = " * `Africa/Casablanca` and `Africa/El_Aaiun` since 2018-10-28"] # [doc = ""] # [doc = " If the TZDB build mode is unknown or variable, use [`TimeZoneInfo::infer_variant`]."] # [deprecated (since = "2.1.0" , note = "TimeZoneVariants don't need to be constructed in user code")] pub const fn from_rearguard_isdst (isdst : bool) -> Self { if isdst { TimeZoneVariant :: Daylight } else { TimeZoneVariant :: Standard } } }
};
}
