// Generated macro for Strategy (trait)
macro_rules! DepcrateStrategy {
() => {
// Module: crate
// Provides: {"Strategy"}
// Dependencies: {}
# [doc = " Controls in which cases the associated code should be run"] pub trait Strategy { # [doc = " Return `true` if the guard’s associated code should run"] # [doc = " (in the context where this method is called)."] fn should_run () -> bool ; }
};
}
