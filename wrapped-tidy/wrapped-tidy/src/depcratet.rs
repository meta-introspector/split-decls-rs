// Generated macro for t (macro)
macro_rules! Depcratet {
() => {
// Module: crate
// Provides: {"t"}
// Dependencies: {}
# [doc = " A helper macro to `unwrap` a result except also print out details like:"] # [doc = ""] # [doc = " * The expression that failed"] # [doc = " * The error itself"] # [doc = " * (optionally) a path connected to the error (e.g. failure to open a file)"] # [macro_export] macro_rules ! t { ($ e : expr , $ p : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed on {} with {}" , stringify ! ($ e) , ($ p) . display () , e) , } } ; ($ e : expr) => { match $ e { Ok (e) => e , Err (e) => panic ! ("{} failed with {}" , stringify ! ($ e) , e) , } } ; }
};
}
