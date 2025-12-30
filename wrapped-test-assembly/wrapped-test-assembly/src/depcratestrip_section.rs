// Generated macro for strip_section (function)
macro_rules! Depcratestrip_section {
() => {
// Module: crate
// Provides: {"strip_section"}
// Dependencies: {}
fn strip_section (data : & str , section : & str) -> String { let mut res = String :: with_capacity (data . len ()) ; let mut in_removed_section = false ; for line in data . lines () { if line . trim () . starts_with (".section") { if line . contains (section) { in_removed_section = true ; println ! ("Stripped {section} section") ; } else { in_removed_section = false ; } } if ! in_removed_section { res . push_str (line) ; res . push ('\n') ; } if line . is_empty () { in_removed_section = false ; } } res }
};
}
