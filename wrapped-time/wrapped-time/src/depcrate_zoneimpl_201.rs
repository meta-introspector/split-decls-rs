// Generated macro for impl_201 (impl)
macro_rules! Depcrate_zoneimpl_201 {
() => {
// Module: crate::zone
// Provides: {"impl_201"}
// Dependencies: {}
impl TimeZone { # [doc = " The synthetic `Etc/Unknown` time zone."] # [doc = ""] # [doc = " This is the result of parsing unknown zones. It's important that such parsing does not"] # [doc = " fail, as new zones are added all the time, and ICU4X might not be up to date."] pub const UNKNOWN : Self = Self (subtag ! ("unk")) ; # [doc = " Whether this [`TimeZone`] equals [`TimeZone::UNKNOWN`]."] pub const fn is_unknown (self) -> bool { matches ! (self , Self :: UNKNOWN) } }
};
}
