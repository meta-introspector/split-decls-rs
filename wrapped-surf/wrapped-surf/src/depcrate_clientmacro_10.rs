// Generated macro for macro_10 (macro)
macro_rules! Depcrate_clientmacro_10 {
() => {
// Module: crate::client
// Provides: {"macro_10"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (feature = "curl-client" , feature = "h1-client" , feature = "h1-client-rustls" , feature = "hyper-client"))] { use once_cell :: sync :: Lazy ; static GLOBAL_CLIENT : Lazy < Arc < DefaultClient >> = Lazy :: new (|| Arc :: new (DefaultClient :: new ())) ; } }
};
}
