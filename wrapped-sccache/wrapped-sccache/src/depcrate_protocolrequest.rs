// Generated macro for Request (enum)
macro_rules! Depcrate_protocolRequest {
() => {
// Module: crate::protocol
// Provides: {"Request"}
// Dependencies: {}
# [doc = " A client request."] # [derive (Serialize , Deserialize , Debug)] pub enum Request { # [doc = " Zero the server's statistics."] ZeroStats , # [doc = " Get server statistics."] GetStats , # [doc = " Get dist status."] DistStatus , # [doc = " Shut the server down gracefully."] Shutdown , # [doc = " Execute a compile or fetch a cached compilation result."] Compile (Compile) , }
};
}
