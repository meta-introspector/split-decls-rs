// Generated macro for impl_645 (impl)
macro_rules! Depcrate_datetime_neo_skeletonimpl_645 {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"impl_645"}
// Dependencies: {}
impl From < & cldr_serde :: ca :: DateTimeFormatsVariant > for GenericLengthPatterns < '_ > { fn from (other : & cldr_serde :: ca :: DateTimeFormatsVariant) -> Self { Self { full : other . standard . full . get_pattern () . parse () . expect ("Failed to parse pattern") , long : other . standard . long . get_pattern () . parse () . expect ("Failed to parse pattern") , medium : other . standard . medium . get_pattern () . parse () . expect ("Failed to parse pattern") , short : other . standard . short . get_pattern () . parse () . expect ("Failed to parse pattern") , } } }
};
}
