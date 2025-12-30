// Generated macro for ServerInfo (struct)
macro_rules! Depcrate_serverServerInfo {
() => {
// Module: crate::server
// Provides: {"ServerInfo"}
// Dependencies: {}
# [doc = " Info and stats about the server."] # [derive (Serialize , Deserialize , Clone , Debug)] pub struct ServerInfo { pub stats : ServerStats , pub cache_location : String , pub cache_size : Option < u64 > , pub max_cache_size : Option < u64 > , pub use_preprocessor_cache_mode : bool , pub version : String , }
};
}
