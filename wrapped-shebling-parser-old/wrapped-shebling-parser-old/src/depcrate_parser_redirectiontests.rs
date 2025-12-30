// Generated macro for tests (module)
macro_rules! Depcrate_parser_redirectiontests {
() => {
// Module: crate::parser::redirection
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: parser :: tests ; # [test] fn test_redir () { assert_parse ! (redir (">foo") , Redir :: new (None , RedirOp :: Great , tests :: word ("foo"))) ; assert_parse ! (redir ("<< foo") , Redir :: new (None , RedirOp :: DLess , tests :: word ("foo"))) ; assert_parse ! (redir ("{foo}>&1-") , Redir :: new (Some (FileDesc :: Var ("foo" . into ())) , RedirOp :: GreatAnd , tests :: word ("1-") ,)) ; assert_parse ! (redir ("{foo}>&{bar}") , Redir :: new (Some (FileDesc :: Var ("foo" . into ())) , RedirOp :: GreatAnd , tests :: word ("bar") ,)) ; assert_parse ! (redir ("{$foo}<<bar") => Err ((1 , 1) , Notes : [((1 , 1) , "invalid file descriptor")])) ; assert_parse ! (redir ("{foo} <<bar") => Err ((1 , 6) , Notes : [((1 , 6) , "expected a redirection operator")])) ; } }
};
}
