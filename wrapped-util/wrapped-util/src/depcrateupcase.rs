// Generated macro for upcase (function)
macro_rules! Depcrateupcase {
() => {
// Module: crate
// Provides: {"upcase"}
// Dependencies: {}
fn upcase (s : & str) -> String { let mut ret = String :: new () ; for ch in s . chars () { if ch == '-' { ret . push ('_') ; } else { for ch in ch . to_uppercase () { ret . push (ch) ; } } } ret }
};
}
