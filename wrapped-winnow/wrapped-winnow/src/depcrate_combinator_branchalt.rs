// Generated macro for Alt (trait)
macro_rules! Depcrate_combinator_branchAlt {
() => {
// Module: crate::combinator::branch
// Provides: {"Alt"}
// Dependencies: {}
# [doc = " Helper trait for the [`alt()`] combinator."] # [doc = ""] # [doc = " This trait is implemented for tuples of up to 21 elements"] pub trait Alt < I , O , E > { # [doc = " Tests each parser in the tuple and returns the result of the first one that succeeds"] fn choice (& mut self , input : & mut I) -> Result < O , E > ; }
};
}
