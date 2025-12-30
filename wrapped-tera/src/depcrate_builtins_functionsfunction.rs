// Generated macro for Function (trait)
macro_rules! Depcrate_builtins_functionsFunction {
() => {
// Module: crate::builtins::functions
// Provides: {"Function"}
// Dependencies: {}
# [doc = " The global function type definition"] pub trait Function : Sync + Send { # [doc = " The global function type definition"] fn call (& self , args : & HashMap < String , Value >) -> Result < Value > ; # [doc = " Whether the current function's output should be treated as safe, defaults to `false`"] fn is_safe (& self) -> bool { false } }
};
}
