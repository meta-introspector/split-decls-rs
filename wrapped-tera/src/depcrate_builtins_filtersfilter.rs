// Generated macro for Filter (trait)
macro_rules! Depcrate_builtins_filtersFilter {
() => {
// Module: crate::builtins::filters
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " The filter function type definition"] pub trait Filter : Sync + Send { # [doc = " The filter function type definition"] fn filter (& self , value : & Value , args : & HashMap < String , Value >) -> Result < Value > ; # [doc = " Whether the current filter's output should be treated as safe, defaults to `false`"] fn is_safe (& self) -> bool { false } }
};
}
