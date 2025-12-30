// Generated macro for impl_208 (impl)
macro_rules! Depcrate_zoneimpl_208 {
() => {
// Module: crate::zone
// Provides: {"impl_208"}
// Dependencies: {}
impl < Model : models :: TimeZoneModel > TimeZoneInfo < Model > { # [doc = " The BCP47 time-zone identifier."] pub fn id (self) -> TimeZone { self . id } # [doc = " The UTC offset, if known."] # [doc = ""] # [doc = " This field is not enforced to be consistent with the time zone id."] pub fn offset (self) -> Option < UtcOffset > { self . offset } }
};
}
