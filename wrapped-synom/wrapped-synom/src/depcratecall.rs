// Generated macro for call (macro)
macro_rules! Depcratecall {
() => {
// Module: crate
// Provides: {"call"}
// Dependencies: {}
# [doc = " Invoke the given parser function with the passed in arguments."] # [doc = ""] # [doc = " - **Syntax:** `call!(FUNCTION, ARGS...)`"] # [doc = ""] # [doc = "   where the signature of the function is `fn(&[U], ARGS...) -> IPResult<&[U], T>`"] # [doc = " - **Output:** `T`, the result of invoking the function `FUNCTION`"] # [macro_export] macro_rules ! call { ($ i : expr , $ fun : expr $ (, $ args : expr) *) => { $ fun ($ i $ (, $ args) *) } ; }
};
}
