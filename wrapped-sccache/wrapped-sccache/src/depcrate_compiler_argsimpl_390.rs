// Generated macro for impl_390 (impl)
macro_rules! Depcrate_compiler_argsimpl_390 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_390"}
// Dependencies: {}
impl < I , T , S > Iterator for ArgsIter < I , T , S > where I : Iterator < Item = OsString > , T : ArgumentValue , S : SearchableArgInfo < T > , { type Item = ArgParseResult < Argument < T > > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (arg) = self . arguments . next () { if let Some (seen_double_dashes) = & mut self . seen_double_dashes { if ! * seen_double_dashes && arg == "--" { * seen_double_dashes = true ; } if * seen_double_dashes { return Some (Ok (Argument :: Raw (arg))) ; } } let s = arg . to_string_lossy () ; let arguments = & mut self . arguments ; Some (match self . arg_info . search (& s [..]) { Some (i) => i . clone () . process (& s [..] , | | arguments . next ()) , None => Ok (if s . starts_with ('-') { Argument :: UnknownFlag (arg . clone ()) } else { Argument :: Raw (arg . clone ()) }) , }) } else { None } } }
};
}
