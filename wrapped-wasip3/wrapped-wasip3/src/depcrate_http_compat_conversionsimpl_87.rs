// Generated macro for impl_87 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_87 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_87"}
// Dependencies: {}
impl TryFrom < http :: HeaderMap > for Fields { type Error = HeaderError ; fn try_from (map : http :: HeaderMap) -> Result < Self , Self :: Error > { let mut last_name = None ; let iter = map . into_iter () . map (move | (name , value) | { if name . is_some () { last_name = name ; } let name = last_name . as_ref () . expect ("HeaderMap::into_iter always returns Some(name) before None") ; let value = bytes :: Bytes :: from_owner (value) . to_vec () ; (name . as_str () . into () , value) }) ; let entries = Vec :: from_iter (iter) ; Fields :: from_list (& entries) } }
};
}
