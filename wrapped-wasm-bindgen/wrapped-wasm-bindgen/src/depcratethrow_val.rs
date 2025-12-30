// Generated macro for throw_val (function)
macro_rules! Depcratethrow_val {
() => {
// Module: crate
// Provides: {"throw_val"}
// Dependencies: {}
# [doc = " Rethrow a JS exception"] # [doc = ""] # [doc = " This function will throw a JS exception with the JS value provided. This"] # [doc = " function will not return and the Wasm stack will be popped until the point"] # [doc = " of entry of Wasm itself."] # [doc = ""] # [doc = " Note that it is very easy to leak memory with this function because this"] # [doc = " function, unlike `panic!` on other platforms, **will not run destructors**."] # [doc = " It's recommended to return a `Result` where possible to avoid the worry of"] # [doc = " leaks."] # [cold] # [inline (never)] pub fn throw_val (s : JsValue) -> ! { __wbindgen_rethrow (s) ; unsafe { core :: hint :: unreachable_unchecked () } }
};
}
