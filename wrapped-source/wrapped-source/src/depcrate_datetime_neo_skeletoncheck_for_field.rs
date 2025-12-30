// Generated macro for check_for_field (function)
macro_rules! Depcrate_datetime_neo_skeletoncheck_for_field {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"check_for_field"}
// Dependencies: {}
# [doc = " An internal function that checks if the attributes contain a field."] fn check_for_field (attributes : & DataMarkerAttributes , field : & str) -> bool { let f0 = field . as_bytes () . first () . unwrap () ; let f1 = field . as_bytes () . get (1) ; let mut it = attributes . as_bytes () . iter () . peekable () ; while let Some (b) = it . next () { if b == f0 { let p = it . peek () ; if p == f1 . as_ref () { return true ; } if field . len () != 1 { return false ; } let Some (q) = p else { return true ; } ; if q . is_ascii_alphabetic () { return true ; } return false ; } } false }
};
}
