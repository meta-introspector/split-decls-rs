// Generated macro for DistClientConfig (struct)
macro_rules! Depcrate_serverDistClientConfig {
() => {
// Module: crate::server
// Provides: {"DistClientConfig"}
// Dependencies: {}
# [cfg (feature = "dist-client")] pub struct DistClientConfig { pool : tokio :: runtime :: Handle , scheduler_url : Option < config :: HTTPUrl > , auth : config :: DistAuth , cache_dir : PathBuf , toolchain_cache_size : u64 , toolchains : Vec < config :: DistToolchainConfig > , rewrite_includes_only : bool , }
};
}
