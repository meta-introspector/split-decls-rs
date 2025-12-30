// Generated macro for fmt_list (function)
macro_rules! Depcratefmt_list {
() => {
// Module: crate
// Provides: {"fmt_list"}
// Dependencies: {}
fn fmt_list < V : std :: fmt :: Debug > (values : impl IntoIterator < Item = V >) -> String { let pieces = values . into_iter () . map (| b | format ! ("{b:?}, ")) . collect :: < Vec < _ > > () ; let mut out = String :: new () ; let mut line = String :: from ("\n    ") ; for piece in pieces { if line . len () + piece . len () < 98 { line . push_str (& piece) ; } else { out . push_str (line . trim_end ()) ; out . push ('\n') ; line = format ! ("    {piece}") ; } } out . push_str (line . trim_end ()) ; out . push ('\n') ; out }
};
}
