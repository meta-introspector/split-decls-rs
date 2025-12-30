// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_configimpl_1009 {
() => {
// Module: crate::config
// Provides: {"impl_1009"}
// Dependencies: {}
impl < 'a > Deserialize < 'a > for DistAuth { fn deserialize < D > (deserializer : D) -> StdResult < Self , D :: Error > where D : Deserializer < 'a > , { # [derive (Deserialize)] # [serde (deny_unknown_fields)] # [serde (tag = "type")] pub enum Helper { # [serde (rename = "token")] Token { token : String } , # [serde (rename = "oauth2_code_grant_pkce")] Oauth2CodeGrantPKCE { client_id : String , auth_url : String , token_url : String , } , # [serde (rename = "oauth2_implicit")] Oauth2Implicit { client_id : String , auth_url : String } , } let helper : Helper = Deserialize :: deserialize (deserializer) ? ; Ok (match helper { Helper :: Token { token } => DistAuth :: Token { token } , Helper :: Oauth2CodeGrantPKCE { client_id , auth_url , token_url , } => DistAuth :: Oauth2CodeGrantPKCE { client_id , auth_url , token_url , } , Helper :: Oauth2Implicit { client_id , auth_url , } => DistAuth :: Oauth2Implicit { client_id , auth_url , } , }) } }
};
}
