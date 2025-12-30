// Generated macro for record_contains_union (function)
macro_rules! Depcraterecord_contains_union {
() => {
// Module: crate
// Provides: {"record_contains_union"}
// Dependencies: {}
fn record_contains_union (s : & RecordDatatype) -> bool { s . members . iter () . any (| member | type_contains_union (& member . tref . type_ ())) }
};
}
