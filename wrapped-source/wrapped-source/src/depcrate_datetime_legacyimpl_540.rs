// Generated macro for impl_540 (impl)
macro_rules! Depcrate_datetime_legacyimpl_540 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_540"}
// Dependencies: {}
impl From < & cldr_serde :: ca :: DateTimeFormats > for LengthPatterns < '_ > { fn from (other : & cldr_serde :: ca :: DateTimeFormats) -> Self { Self { medium : other . medium . get_pattern () . parse () . expect ("Failed to parse pattern") , short : other . short . get_pattern () . parse () . expect ("Failed to parse pattern") , } } }
};
}
