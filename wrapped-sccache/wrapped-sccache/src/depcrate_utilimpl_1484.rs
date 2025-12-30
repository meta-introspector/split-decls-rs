// Generated macro for impl_1484 (impl)
macro_rules! Depcrate_utilimpl_1484 {
() => {
// Module: crate::util
// Provides: {"impl_1484"}
// Dependencies: {}
# [cfg (unix)] impl OsStrExt for OsStr { fn starts_with (& self , s : & str) -> bool { self . as_bytes () . starts_with (s . as_bytes ()) } fn split_prefix (& self , s : & str) -> Option < OsString > { let bytes = self . as_bytes () ; if bytes . starts_with (s . as_bytes ()) { Some (OsStr :: from_bytes (& bytes [s . len () ..]) . to_owned ()) } else { None } } }
};
}
