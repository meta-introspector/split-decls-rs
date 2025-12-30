// Generated macro for impl_210 (impl)
macro_rules! Depcrate_zoneimpl_210 {
() => {
// Module: crate::zone
// Provides: {"impl_210"}
// Dependencies: {}
impl < Model > TimeZoneInfo < Model > where Model : models :: TimeZoneModel < TimeZoneVariant = TimeZoneVariant > , { # [doc = " The time variant e.g. daylight or standard, if known."] # [doc = ""] # [doc = " This field is not enforced to be consistent with the time zone id and offset."] pub fn variant (self) -> TimeZoneVariant { self . variant } }
};
}
