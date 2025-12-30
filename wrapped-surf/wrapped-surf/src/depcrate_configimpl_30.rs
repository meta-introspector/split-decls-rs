// Generated macro for impl_30 (impl)
macro_rules! Depcrate_configimpl_30 {
() => {
// Module: crate::config
// Provides: {"impl_30"}
// Dependencies: {}
impl From < HttpConfig > for Config { fn from (http_config : HttpConfig) -> Self { Self { base_url : None , headers : HashMap :: new () , http_config , http_client : None , } } }
};
}
