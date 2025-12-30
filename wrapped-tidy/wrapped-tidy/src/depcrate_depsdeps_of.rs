// Generated macro for deps_of (function)
macro_rules! Depcrate_depsdeps_of {
() => {
// Module: crate::deps
// Provides: {"deps_of"}
// Dependencies: {}
# [doc = " Recursively find all dependencies."] fn deps_of < 'a > (metadata : & 'a Metadata , pkg_id : & 'a PackageId , result : & mut HashSet < & 'a PackageId >) { if ! result . insert (pkg_id) { return ; } let node = metadata . resolve . as_ref () . unwrap () . nodes . iter () . find (| n | & n . id == pkg_id) . unwrap_or_else (| | panic ! ("could not find `{pkg_id}` in resolve")) ; for dep in & node . deps { deps_of (metadata , & dep . pkg , result) ; } }
};
}
