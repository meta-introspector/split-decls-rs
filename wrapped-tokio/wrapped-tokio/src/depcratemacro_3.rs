// Generated macro for macro_3 (macro)
macro_rules! Depcratemacro_3 {
() => {
// Module: crate
// Provides: {"macro_3"}
// Dependencies: {}
# [cfg (all (not (tokio_unstable) , feature = "io-uring"))] compile_error ! ("The `io-uring` feature requires `--cfg tokio_unstable`.") ;
};
}
