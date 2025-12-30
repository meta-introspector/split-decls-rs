// Generated macro for impl_386 (impl)
macro_rules! Depcrate_compiler_argsimpl_386 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_386"}
// Dependencies: {}
# [doc = " Allow to search over a sorted array of ArgInfo items associated with extra"] # [doc = " data."] impl < T : ArgumentValue > SearchableArgInfo < T > for & 'static [ArgInfo < T >] { fn search (& self , key : & str) -> Option < & ArgInfo < T > > { bsearch (key , self , | i , k | i . cmp (k)) } # [cfg (debug_assertions)] fn check (& self) -> bool { self . windows (2) . all (| w | { let a = w [0] . flag_str () ; let b = w [1] . flag_str () ; assert ! (a < b , "{} can't precede {}" , a , b) ; true }) } }
};
}
