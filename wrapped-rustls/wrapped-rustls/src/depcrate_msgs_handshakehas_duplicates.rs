// Generated macro for has_duplicates (function)
macro_rules! Depcrate_msgs_handshakehas_duplicates {
() => {
// Module: crate::msgs::handshake
// Provides: {"has_duplicates"}
// Dependencies: {}
fn has_duplicates < I : IntoIterator < Item = E > , E : Into < T > , T : Eq + Ord > (iter : I) -> bool { let mut seen = BTreeSet :: new () ; for x in iter { if ! seen . insert (x . into ()) { return true ; } } false }
};
}
