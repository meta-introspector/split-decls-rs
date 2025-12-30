// Generated macro for config_file (function)
macro_rules! Depcrate_configconfig_file {
() => {
// Module: crate::config
// Provides: {"config_file"}
// Dependencies: {}
fn config_file (env_var : & str , leaf : & str) -> PathBuf { if let Some (env_value) = env :: var_os (env_var) { return env_value . into () ; } let dirs = ProjectDirs :: from ("" , ORGANIZATION , APP_NAME) . expect ("Unable to get config directory") ; let path = dirs . config_dir () . join (leaf) ; if path . exists () { return path ; } let path = dirs . preference_dir () . join (leaf) ; if path . exists () { return path ; } dirs . config_dir () . join (leaf) }
};
}
