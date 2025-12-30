// Generated macro for osstrings_to_strings (function)
macro_rules! Depcrate_distosstrings_to_strings {
() => {
// Module: crate::dist
// Provides: {"osstrings_to_strings"}
// Dependencies: {}
pub fn osstrings_to_strings (osstrings : & [OsString]) -> Option < Vec < String > > { osstrings . iter () . map (| arg | arg . clone () . into_string () . ok ()) . collect :: < Option < _ > > () }
};
}
