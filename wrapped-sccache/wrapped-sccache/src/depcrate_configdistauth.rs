// Generated macro for DistAuth (enum)
macro_rules! Depcrate_configDistAuth {
() => {
// Module: crate::config
// Provides: {"DistAuth"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Serialize)] # [serde (tag = "type")] pub enum DistAuth { # [serde (rename = "token")] Token { token : String } , # [serde (rename = "oauth2_code_grant_pkce")] Oauth2CodeGrantPKCE { client_id : String , auth_url : String , token_url : String , } , # [serde (rename = "oauth2_implicit")] Oauth2Implicit { client_id : String , auth_url : String } , }
};
}
