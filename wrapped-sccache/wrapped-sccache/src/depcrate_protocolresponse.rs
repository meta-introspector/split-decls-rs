// Generated macro for Response (enum)
macro_rules! Depcrate_protocolResponse {
() => {
// Module: crate::protocol
// Provides: {"Response"}
// Dependencies: {}
# [doc = " A server response."] # [derive (Serialize , Deserialize , Debug)] pub enum Response { # [doc = " Response for `Request::Compile`."] Compile (CompileResponse) , # [doc = " Response for `Request::ZeroStats`."] ZeroStats , # [doc = " Response for `Request::GetStats`, containing server statistics."] Stats (Box < ServerInfo >) , # [doc = " Response for `Request::DistStatus`, containing client info."] DistStatus (DistInfo) , # [doc = " Response for `Request::Shutdown`, containing server statistics."] ShuttingDown (Box < ServerInfo >) , # [doc = " Second response for `Request::Compile`, containing the results of the compilation."] CompileFinished (CompileFinished) , }
};
}
