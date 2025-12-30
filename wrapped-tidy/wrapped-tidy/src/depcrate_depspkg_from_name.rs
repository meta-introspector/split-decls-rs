// Generated macro for pkg_from_name (function)
macro_rules! Depcrate_depspkg_from_name {
() => {
// Module: crate::deps
// Provides: {"pkg_from_name"}
// Dependencies: {}
# [doc = " Finds a package with the given name."] fn pkg_from_name < 'a > (metadata : & 'a Metadata , name : & 'static str) -> & 'a Package { let mut i = metadata . packages . iter () . filter (| p | * p . name == name) ; let result = i . next () . unwrap_or_else (| | panic ! ("could not find package `{name}` in package list")) ; assert ! (i . next () . is_none () , "more than one package found for `{name}`") ; result }
};
}
