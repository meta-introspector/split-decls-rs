// Generated macro for ExpressionExt (trait)
macro_rules! DepcrateExpressionExt {
() => {
// Module: crate
// Provides: {"ExpressionExt"}
// Dependencies: {}
# [doc = " An extension trait for `duct::Expression` that logs the command being run"] # [doc = " before running it."] trait ExpressionExt { # [doc = " Run the command and log the command being run"] fn run_with_trace (& self) -> io :: Result < Output > ; }
};
}
