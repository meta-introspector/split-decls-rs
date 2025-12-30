// Generated macro for USAGE (const)
macro_rules! DepcrateUSAGE {
() => {
// Module: crate
// Provides: {"USAGE"}
// Dependencies: {}
const USAGE : & str = "Usage:

    symbol-check build-and-check [TARGET] -- CARGO_BUILD_ARGS ...

Cargo will get invoked with `CARGO_ARGS` and the specified target. All output
`compiler_builtins*.rlib` files will be checked.

If TARGET is not specified, the host target is used.

    check PATHS ...

Run the same checks on the given set of paths, without invoking Cargo. Paths
may be either archives or object files.
" ;
};
}
