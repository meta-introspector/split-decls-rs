// Generated macro for black_box (function)
macro_rules! Depcrateblack_box {
() => {
// Module: crate
// Provides: {"black_box"}
// Dependencies: {}
# [doc = " This function is a best-effort attempt to prevent the compiler from knowing"] # [doc = " anything about the value of the returned `u8`, other than its type."] # [doc = ""] # [doc = " Because we want to support stable Rust, we don't have access to inline"] # [doc = " assembly or test::black_box, so we use the fact that volatile values will"] # [doc = " never be elided to register values."] # [doc = ""] # [doc = " Note: Rust's notion of \"volatile\" is subject to change over time. While this"] # [doc = " code may break in a non-destructive way in the future, “constant-time” code"] # [doc = " is a continually moving target, and this is better than doing nothing."] # [inline (never)] fn black_box < T : Copy > (input : T) -> T { unsafe { core :: ptr :: read_volatile (& input) } }
};
}
