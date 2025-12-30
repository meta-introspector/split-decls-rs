// Generated macro for cond (macro)
macro_rules! Depcratecond {
() => {
// Module: crate
// Provides: {"cond"}
// Dependencies: {}
# [doc = " Conditionally execute the given parser."] # [doc = ""] # [doc = " If you are familiar with nom, this is nom's `cond_with_error` parser."] # [doc = ""] # [doc = " - **Syntax:** `cond!(CONDITION, THING)`"] # [doc = " - **Output:** `Some(THING)` if the condition is true, else `None`"] # [macro_export] macro_rules ! cond { ($ i : expr , $ cond : expr , $ submac : ident ! ($ ($ args : tt) *)) => { if $ cond { match $ submac ! ($ i , $ ($ args) *) { :: std :: result :: Result :: Ok ((i , o)) => :: std :: result :: Result :: Ok ((i , :: std :: option :: Option :: Some (o))) , :: std :: result :: Result :: Err (x) => :: std :: result :: Result :: Err (x) , } } else { :: std :: result :: Result :: Ok (($ i , :: std :: option :: Option :: None)) } } ; ($ i : expr , $ cond : expr , $ f : expr) => { cond ! ($ i , $ cond , call ! ($ f)) } ; }
};
}
