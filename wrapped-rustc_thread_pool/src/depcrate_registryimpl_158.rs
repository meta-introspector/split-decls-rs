// Generated macro for impl_158 (impl)
macro_rules! Depcrate_registryimpl_158 {
() => {
// Module: crate::registry
// Provides: {"impl_158"}
// Dependencies: {}
impl ThreadSpawn for DefaultSpawn { private_impl ! { } fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { let mut b = thread :: Builder :: new () ; if let Some (name) = thread . name () { b = b . name (name . to_owned ()) ; } if let Some (stack_size) = thread . stack_size () { b = b . stack_size (stack_size) ; } b . spawn (| | thread . run ()) ? ; Ok (()) } }
};
}
