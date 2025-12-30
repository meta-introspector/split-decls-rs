// Generated macro for ThreadSpawn (trait)
macro_rules! Depcrate_registryThreadSpawn {
() => {
// Module: crate::registry
// Provides: {"ThreadSpawn"}
// Dependencies: {}
# [doc = " Generalized trait for spawning a thread in the `Registry`."] # [doc = ""] # [doc = " This trait is pub-in-private -- E0445 forces us to make it public,"] # [doc = " but we don't actually want to expose these details in the API."] pub trait ThreadSpawn { private_decl ! { } # [doc = " Spawn a thread with the `ThreadBuilder` parameters, and then"] # [doc = " call `ThreadBuilder::run()`."] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > ; }
};
}
