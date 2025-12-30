// Generated macro for impl_985 (impl)
macro_rules! Depcrate_configimpl_985 {
() => {
// Module: crate::config
// Provides: {"impl_985"}
// Dependencies: {}
# [cfg (any (feature = "dist-client" , feature = "dist-server"))] impl < 'a > Deserialize < 'a > for HTTPUrl { fn deserialize < D > (deserializer : D) -> StdResult < Self , D :: Error > where D : Deserializer < 'a > , { use serde :: de :: Error ; let helper : String = Deserialize :: deserialize (deserializer) ? ; let url = parse_http_url (& helper) . map_err (D :: Error :: custom) ? ; Ok (HTTPUrl (url)) } }
};
}
