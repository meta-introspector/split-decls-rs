// Generated macro for impl_368 (impl)
macro_rules! Depcrate_compiler_argsimpl_368 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_368"}
// Dependencies: {}
# [cfg (feature = "dist-client")] impl < T : ArgumentValue , F : FnMut (& Path) -> Option < String > > Iterator for IterStrings < '_ , T , F > { type Item = ArgToStringResult ; fn next (& mut self) -> Option < Self :: Item > { let result : Option < Self :: Item > = match * self . arg { Argument :: Raw (ref s) | Argument :: UnknownFlag (ref s) => match self . emitted { 0 => Some (s . clone () . into_arg_string (& mut self . path_transformer)) , _ => None , } , Argument :: Flag (s , _) => match self . emitted { 0 => Some (Ok (s . to_owned ())) , _ => None , } , Argument :: WithValue (s , ref v , ref d) => match (self . emitted , d) { (0 , & ArgDisposition :: CanBeSeparated (d)) | (0 , & ArgDisposition :: Concatenated (d)) => { let mut s = s . to_owned () ; let v = match v . clone () . into_arg_string (& mut self . path_transformer) { Ok (s) => s , Err (e) => return Some (Err (e)) , } ; if let Some (d) = d { if ! v . is_empty () { s . push_str (str :: from_utf8 (& [d]) . expect ("delimiter should be ascii")) ; } } s . push_str (& v) ; Some (Ok (s)) } (0 , & ArgDisposition :: Separated) | (0 , & ArgDisposition :: CanBeConcatenated (_)) => { Some (Ok (s . to_owned ())) } (1 , & ArgDisposition :: Separated) | (1 , & ArgDisposition :: CanBeConcatenated (_)) => { Some (v . clone () . into_arg_string (& mut self . path_transformer)) } _ => None , } , } ; if result . is_some () { self . emitted += 1 ; } result } }
};
}
