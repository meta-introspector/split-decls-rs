// Generated macro for impl_642 (impl)
macro_rules! Depcrate_compiler_diabimpl_642 {
() => {
// Module: crate::compiler::diab
// Provides: {"impl_642"}
// Dependencies: {}
impl Iterator for ExpandAtArgs < '_ > { type Item = OsString ; fn next (& mut self) -> Option < OsString > { loop { let arg = self . stack . pop () ? ; if ! arg . starts_with ("-@") { return Some (arg) ; } let value = match arg . split_prefix ("-@") { Some (arg) => arg , None => return Some (arg) , } ; if value . starts_with ("E") || value . starts_with ("O") || value . starts_with ("@") { return Some (arg) ; } let mut contents = String :: new () ; let file = self . cwd . join (& value) ; let res = File :: open (file) . and_then (| mut f | f . read_to_string (& mut contents)) ; if res . is_err () { return Some (arg) ; } if contents . contains ('"') || contents . contains ('\'') { return Some (arg) ; } let new_args = contents . split_whitespace () . collect :: < Vec < _ > > () ; self . stack . extend (new_args . iter () . rev () . map (| s | s . into ())) ; } } }
};
}
