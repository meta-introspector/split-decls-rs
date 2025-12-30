// Generated macro for macro_9 (macro)
macro_rules! Depcrate_clientmacro_9 {
() => {
// Module: crate::client
// Provides: {"macro_9"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "curl-client")] { use http_client :: isahc :: IsahcClient as DefaultClient ; } else if # [cfg (feature = "wasm-client")] { use http_client :: wasm :: WasmClient as DefaultClient ; } else if # [cfg (any (feature = "h1-client" , feature = "h1-client-rustls" , feature = "h1-client-no-tls"))] { use http_client :: h1 :: H1Client as DefaultClient ; } else if # [cfg (feature = "hyper-client")] { use http_client :: hyper :: HyperClient as DefaultClient ; } }
};
}
