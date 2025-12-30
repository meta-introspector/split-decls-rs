// Generated macro for human_units_parse (function)
macro_rules! Depcrate_confighuman_units_parse {
() => {
// Module: crate::config
// Provides: {"human_units_parse"}
// Dependencies: {}
# [test] fn human_units_parse () { const CONFIG_STR : & str = r#"
[dist]
toolchain_cache_size = "5g"

[cache.disk]
size = "7g"
"# ; let file_config : FileConfig = toml :: from_str (CONFIG_STR) . expect ("Is valid toml.") ; assert_eq ! (file_config , FileConfig { cache : CacheConfigs { disk : Some (DiskCacheConfig { size : 7 * 1024 * 1024 * 1024 , .. Default :: default () }) , .. Default :: default () } , dist : DistConfig { toolchain_cache_size : 5 * 1024 * 1024 * 1024 , .. Default :: default () } , server_startup_timeout_ms : None , }) ; }
};
}
