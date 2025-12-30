// Generated macro for osstring_tuples_to_strings (function)
macro_rules! Depcrate_distosstring_tuples_to_strings {
() => {
// Module: crate::dist
// Provides: {"osstring_tuples_to_strings"}
// Dependencies: {}
pub fn osstring_tuples_to_strings (osstring_tuples : & [(OsString , OsString)] ,) -> Option < Vec < (String , String) > > { osstring_tuples . iter () . map (| (k , v) | Some ((k . clone () . into_string () . ok () ? , v . clone () . into_string () . ok () ?))) . collect :: < Option < _ > > () }
};
}
