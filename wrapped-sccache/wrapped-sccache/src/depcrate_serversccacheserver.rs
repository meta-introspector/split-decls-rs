// Generated macro for SccacheServer (struct)
macro_rules! Depcrate_serverSccacheServer {
() => {
// Module: crate::server
// Provides: {"SccacheServer"}
// Dependencies: {}
pub struct SccacheServer < A : crate :: net :: Acceptor , C : CommandCreatorSync = ProcessCommandCreator > { runtime : Runtime , listener : A , rx : mpsc :: Receiver < ServerMessage > , timeout : Duration , service : SccacheService < C > , wait : WaitUntilZero , }
};
}
