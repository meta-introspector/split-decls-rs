// Generated macro for check_cargo_toml (function)
macro_rules! Depcrate_tidycheck_cargo_toml {
() => {
// Module: crate::tidy
// Provides: {"check_cargo_toml"}
// Dependencies: {}
fn check_cargo_toml (path : & Path , text : String) { let mut section = None ; for (line_no , text) in text . lines () . enumerate () { let text = text . trim () ; if text . starts_with ('[') { if ! text . ends_with (']') { panic ! ("\nplease don't add comments or trailing whitespace in section lines.\n\
                        {}:{}\n" , path . display () , line_no + 1) } section = Some (text) ; continue ; } let text : String = text . split_whitespace () . collect () ; if ! text . contains ("path=") { continue ; } match section { Some (s) if s . contains ("dev-dependencies") => { if text . contains ("version") { panic ! ("\ncargo internal dev-dependencies should not have a version.\n\
                        {}:{}\n" , path . display () , line_no + 1) ; } } Some (s) if s . contains ("dependencies") => { if ! text . contains ("version") { panic ! ("\ncargo internal dependencies should have a version.\n\
                        {}:{}\n" , path . display () , line_no + 1) ; } } _ => { } } } }
};
}
