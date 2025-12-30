// Generated macro for impl_711 (impl)
macro_rules! Depcrate_compiler_msvcimpl_711 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_711"}
// Dependencies: {}
impl Iterator for ExpandIncludeFile < '_ > { type Item = OsString ; fn next (& mut self) -> Option < OsString > { loop { if let Some (response_file_arg) = self . stack . pop () { return Some (response_file_arg) ; } let arg = self . args . pop () ? ; let file_arg = match arg . split_prefix ("@") { Some (file_arg) => file_arg , None => return Some (arg) , } ; let file_path = self . cwd . join (file_arg) ; let content = match File :: open (& file_path) . and_then (| mut file | read_text (& mut file)) { Ok (content) => content , Err (err) => { debug ! ("failed to read @-file `{}`: {}" , file_path . display () , err) ; return Some (arg) ; } } ; trace ! ("Expanded response file {:?} to {:?}" , file_path , content) ; let resp_file_args = SplitMsvcResponseFileArgs :: from (& content) . collect :: < Vec < _ > > () ; let rev_args = resp_file_args . iter () . rev () . map (| s | s . into ()) ; self . stack . extend (rev_args) ; } } }
};
}
