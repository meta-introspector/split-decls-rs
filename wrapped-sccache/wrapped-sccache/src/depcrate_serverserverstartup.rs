// Generated macro for ServerStartup (enum)
macro_rules! Depcrate_serverServerStartup {
() => {
// Module: crate::server
// Provides: {"ServerStartup"}
// Dependencies: {}
# [doc = " Result of background server startup."] # [derive (Debug , Serialize , Deserialize)] pub enum ServerStartup { # [doc = " Server started successfully on `addr`."] Ok { addr : String } , # [doc = " Server Addr already in suse"] AddrInUse , # [doc = " Timed out waiting for server startup."] TimedOut , # [doc = " Server encountered an error."] Err { reason : String } , }
};
}
