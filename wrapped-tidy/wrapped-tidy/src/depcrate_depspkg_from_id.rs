// Generated macro for pkg_from_id (function)
macro_rules! Depcrate_depspkg_from_id {
() => {
// Module: crate::deps
// Provides: {"pkg_from_id"}
// Dependencies: {}
fn pkg_from_id < 'a > (metadata : & 'a Metadata , id : & PackageId) -> & 'a Package { metadata . packages . iter () . find (| p | & p . id == id) . unwrap () }
};
}
