// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl ExpressionExt for duct :: Expression { fn run_with_trace (& self) -> io :: Result < Output > { tracing :: info ! ("running command: {:?}" , self) ; self . run () . inspect_err (| _ | { tracing :: error ! ("failed to run command: {:?}" , self) ; }) } }
};
}
