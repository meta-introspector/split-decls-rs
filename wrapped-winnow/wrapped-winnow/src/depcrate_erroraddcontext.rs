// Generated macro for AddContext (trait)
macro_rules! Depcrate_errorAddContext {
() => {
// Module: crate::error
// Provides: {"AddContext"}
// Dependencies: {}
# [doc = " Used by [`Parser::context`] to add custom data to error while backtracking"] # [doc = ""] # [doc = " May be implemented multiple times for different kinds of context."] pub trait AddContext < I : Stream , C = & 'static str > : Sized { # [doc = " Append to an existing error custom data"] # [doc = ""] # [doc = " This is used mainly by [`Parser::context`], to add user friendly information"] # [doc = " to errors when backtracking through a parse tree"] # [inline] fn add_context (self , _input : & I , _token_start : & < I as Stream > :: Checkpoint , _context : C ,) -> Self { self } }
};
}
