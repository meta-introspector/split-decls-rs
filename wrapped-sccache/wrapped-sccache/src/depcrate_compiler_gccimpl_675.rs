// Generated macro for impl_675 (impl)
macro_rules! Depcrate_compiler_gccimpl_675 {
() => {
// Module: crate::compiler::gcc
// Provides: {"impl_675"}
// Dependencies: {}
impl Iterator for ExpandIncludeFile < '_ > { type Item = OsString ; fn next (& mut self) -> Option < OsString > { loop { let arg = self . stack . pop () ? ; let file = match arg . split_prefix ("@") { Some (arg) => self . cwd . join (arg) , None => return Some (arg) , } ; let mut contents = String :: new () ; let res = File :: open (& file) . and_then (| mut f | f . read_to_string (& mut contents)) ; if let Err (e) = res { debug ! ("failed to read @-file `{}`: {}" , file . display () , e) ; return Some (arg) ; } if contents . contains ('"') || contents . contains ('\'') { return Some (arg) ; } let new_args = contents . split_whitespace () . collect :: < Vec < _ > > () ; self . stack . extend (new_args . iter () . rev () . map (| s | s . into ())) ; } } }
};
}
