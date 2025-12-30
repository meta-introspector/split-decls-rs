// Generated macro for impl_387 (impl)
macro_rules! Depcrate_compiler_argsimpl_387 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_387"}
// Dependencies: {}
# [doc = " Allow to search over a couple of arrays of ArgInfo, where the second"] # [doc = " complements or overrides the first one."] impl < T : ArgumentValue > SearchableArgInfo < T > for (& 'static [ArgInfo < T >] , & 'static [ArgInfo < T >]) { fn search (& self , key : & str) -> Option < & ArgInfo < T > > { match (self . 0 . search (key) , self . 1 . search (key)) { (None , None) => None , (Some (a) , None) => Some (a) , (None , Some (a)) => Some (a) , (Some (a) , Some (b)) => { if a . flag_str () > b . flag_str () { Some (a) } else { Some (b) } } } } # [cfg (debug_assertions)] fn check (& self) -> bool { self . 0 . check () && self . 1 . check () } }
};
}
