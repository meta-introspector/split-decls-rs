// Generated macro for Always (enum)
macro_rules! DepcrateAlways {
() => {
// Module: crate
// Provides: {"Always"}
// Dependencies: {}
# [doc = " Always run on scope exit."] # [doc = ""] # [doc = " “Always” run: on regular exit from a scope or on unwinding from a panic."] # [doc = " Can not run on abort, process exit, and other catastrophic events where"] # [doc = " destructors don’t run."] # [derive (Debug)] pub enum Always { }
};
}
