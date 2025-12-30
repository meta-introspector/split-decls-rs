// Generated macro for impl_153 (impl)
macro_rules! Depcrate_lifetimeimpl_153 {
() => {
// Module: crate::lifetime
// Provides: {"impl_153"}
// Dependencies: {}
impl Lifetime { pub fn new (sym : Term , span : Span) -> Self { let s = sym . as_str () ; if ! s . starts_with ('\'') { panic ! ("lifetime name must start with apostrophe as in \"'a\", \
                   got {:?}" , s) ; } if s == "'" { panic ! ("lifetime name must not be empty") ; } if s == "'_" { panic ! ("\"'_\" is not a valid lifetime name") ; } fn xid_ok (s : & str) -> bool { let mut chars = s . chars () ; let first = chars . next () . unwrap () ; if ! (UnicodeXID :: is_xid_start (first) || first == '_') { return false ; } for ch in chars { if ! UnicodeXID :: is_xid_continue (ch) { return false ; } } true } if ! xid_ok (& s [1 ..]) { panic ! ("{:?} is not a valid lifetime name") ; } Lifetime { sym : sym , span : span , } } }
};
}
