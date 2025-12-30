// Generated macro for impl_366 (impl)
macro_rules! Depcrate_compiler_argsimpl_366 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_366"}
// Dependencies: {}
impl < T : ArgumentValue > Iterator for Iter < '_ , T > { type Item = OsString ; fn next (& mut self) -> Option < Self :: Item > { let result = match * self . arg { Argument :: Raw (ref s) | Argument :: UnknownFlag (ref s) => match self . emitted { 0 => Some (s . clone ()) , _ => None , } , Argument :: Flag (s , _) => match self . emitted { 0 => Some (s . into ()) , _ => None , } , Argument :: WithValue (s , ref v , ref d) => match (self . emitted , d) { (0 , & ArgDisposition :: CanBeSeparated (d)) | (0 , & ArgDisposition :: Concatenated (d)) => { let mut s = OsString :: from (s) ; let v = v . clone () . into_arg_os_string () ; if let Some (d) = d { if ! v . is_empty () { s . push (OsString :: from (str :: from_utf8 (& [d]) . expect ("delimiter should be ascii") ,)) ; } } s . push (v) ; Some (s) } (0 , & ArgDisposition :: Separated) | (0 , & ArgDisposition :: CanBeConcatenated (_)) => { Some (s . into ()) } (1 , & ArgDisposition :: Separated) | (1 , & ArgDisposition :: CanBeConcatenated (_)) => { Some (v . clone () . into_arg_os_string ()) } _ => None , } , } ; if result . is_some () { self . emitted += 1 ; } result } }
};
}
