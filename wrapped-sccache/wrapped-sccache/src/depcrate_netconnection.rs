// Generated macro for Connection (trait)
macro_rules! Depcrate_netConnection {
() => {
// Module: crate::net
// Provides: {"Connection"}
// Dependencies: {}
pub trait Connection : std :: io :: Read + std :: io :: Write { fn try_clone (& self) -> std :: io :: Result < Box < dyn Connection > > ; }
};
}
