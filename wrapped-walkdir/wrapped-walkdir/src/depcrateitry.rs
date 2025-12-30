// Generated macro for itry (macro)
macro_rules! Depcrateitry {
() => {
// Module: crate
// Provides: {"itry"}
// Dependencies: {}
# [doc = " Like try, but for iterators that return [`Option<Result<_, _>>`]."] # [doc = ""] # [doc = " [`Option<Result<_, _>>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html"] macro_rules ! itry { ($ e : expr) => { match $ e { Ok (v) => v , Err (err) => return Some (Err (From :: from (err))) , } } ; }
};
}
