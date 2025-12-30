// Generated macro for throw_str (function)
macro_rules! Depcratethrow_str {
() => {
// Module: crate
// Provides: {"throw_str"}
// Dependencies: {}
# [doc = " Throws a JS exception."] # [doc = ""] # [doc = " This function will throw a JS exception with the message provided. The"] # [doc = " function will not return as the Wasm stack will be popped when the exception"] # [doc = " is thrown."] # [doc = ""] # [doc = " Note that it is very easy to leak memory with this function because this"] # [doc = " function, unlike `panic!` on other platforms, **will not run destructors**."] # [doc = " It's recommended to return a `Result` where possible to avoid the worry of"] # [doc = " leaks."] # [cold] # [inline (never)] pub fn throw_str (s : & str) -> ! { __wbindgen_throw (s) ; unsafe { core :: hint :: unreachable_unchecked () } }
};
}
