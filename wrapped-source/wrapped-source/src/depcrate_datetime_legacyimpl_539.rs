// Generated macro for impl_539 (impl)
macro_rules! Depcrate_datetime_legacyimpl_539 {
() => {
// Module: crate::datetime::legacy
// Provides: {"impl_539"}
// Dependencies: {}
impl From < & cldr_serde :: ca :: LengthPatterns > for LengthPatterns < '_ > { fn from (other : & cldr_serde :: ca :: LengthPatterns) -> Self { Self { medium : other . medium . get_pattern () . parse () . expect ("Failed to parse pattern") , short : other . short . get_pattern () . parse () . expect ("Failed to parse pattern") , } } }
};
}
