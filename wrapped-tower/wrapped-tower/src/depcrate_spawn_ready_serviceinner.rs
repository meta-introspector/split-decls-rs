// Generated macro for Inner (enum)
macro_rules! Depcrate_spawn_ready_serviceInner {
() => {
// Module: crate::spawn_ready::service
// Provides: {"Inner"}
// Dependencies: {}
# [derive (Debug)] enum Inner < S > { Service (Option < S >) , Future (tokio :: task :: JoinHandle < Result < S , BoxError > >) , }
};
}
