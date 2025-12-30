// Generated macro for rustdoc (function)
macro_rules! Depcraterustdoc {
() => {
// Module: crate
// Provides: {"rustdoc"}
// Dependencies: {}
fn rustdoc (docs : & str , dst : & mut String) { if docs . trim () . is_empty () { return ; } for line in docs . lines () { dst . push_str ("/// ") ; dst . push_str (line) ; dst . push_str ("\n") ; } }
};
}
