// Generated macro for SearchableArgInfo (trait)
macro_rules! Depcrate_compiler_argsSearchableArgInfo {
() => {
// Module: crate::compiler::args
// Provides: {"SearchableArgInfo"}
// Dependencies: {}
# [doc = " Trait for generically search over a \"set\" of ArgInfos."] pub trait SearchableArgInfo < T > { fn search (& self , key : & str) -> Option < & ArgInfo < T > > ; # [cfg (debug_assertions)] fn check (& self) -> bool ; }
};
}
