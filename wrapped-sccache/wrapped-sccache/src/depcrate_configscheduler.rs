// Generated macro for scheduler (module)
macro_rules! Depcrate_configscheduler {
() => {
// Module: crate::config
// Provides: {"scheduler"}
// Dependencies: {}
# [cfg (feature = "dist-server")] pub mod scheduler { use std :: net :: SocketAddr ; use std :: path :: Path ; use crate :: errors :: * ; use serde :: { Deserialize , Serialize } ; # [derive (Debug , Serialize , Deserialize)] # [serde (tag = "type")] # [serde (deny_unknown_fields)] pub enum ClientAuth { # [serde (rename = "DANGEROUSLY_INSECURE")] Insecure , # [serde (rename = "token")] Token { token : String } , # [serde (rename = "jwt_validate")] JwtValidate { audience : String , issuer : String , jwks_url : String , } , # [serde (rename = "proxy_token")] ProxyToken { url : String , cache_secs : Option < u64 > , } , } # [derive (Debug , Serialize , Deserialize)] # [serde (tag = "type")] # [serde (deny_unknown_fields)] pub enum ServerAuth { # [serde (rename = "DANGEROUSLY_INSECURE")] Insecure , # [serde (rename = "jwt_hs256")] JwtHS256 { secret_key : String } , # [serde (rename = "token")] Token { token : String } , } # [derive (Debug , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub struct Config { pub public_addr : SocketAddr , pub client_auth : ClientAuth , pub server_auth : ServerAuth , } pub fn from_path (conf_path : & Path) -> Result < Option < Config > > { super :: try_read_config_file (conf_path) . context ("Failed to load scheduler config file") } }
};
}
