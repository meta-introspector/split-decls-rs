// Generated macro for Error (trait)
macro_rules! Depcrate_bridgeError {
() => {
// Module: crate::bridge
// Provides: {"Error"}
// Dependencies: {}
pub trait Error { fn new (msg : String) -> Self ; fn from_internal < T : Debug > (err : T) -> Self ; }
};
}
