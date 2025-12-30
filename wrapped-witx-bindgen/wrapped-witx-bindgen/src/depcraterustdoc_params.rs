// Generated macro for rustdoc_params (function)
macro_rules! Depcraterustdoc_params {
() => {
// Module: crate
// Provides: {"rustdoc_params"}
// Dependencies: {}
fn rustdoc_params (docs : & [InterfaceFuncParam] , header : & str , dst : & mut String) { let docs = docs . iter () . filter (| param | param . docs . trim () . len () > 0) . collect :: < Vec < _ > > () ; if docs . len () == 0 { return ; } dst . push_str ("///\n") ; dst . push_str ("/// ## ") ; dst . push_str (header) ; dst . push_str ("\n") ; dst . push_str ("///\n") ; for param in docs { for (i , line) in param . docs . lines () . enumerate () { dst . push_str ("/// ") ; if header != "Return" { if i == 0 { dst . push_str ("* `") ; param . name . render (dst) ; dst . push_str ("` - ") ; } else { dst . push_str ("  ") ; } } dst . push_str (line) ; dst . push_str ("\n") ; } } }
};
}
