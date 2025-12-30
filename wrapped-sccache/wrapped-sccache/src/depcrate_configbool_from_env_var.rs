// Generated macro for bool_from_env_var (function)
macro_rules! Depcrate_configbool_from_env_var {
() => {
// Module: crate::config
// Provides: {"bool_from_env_var"}
// Dependencies: {}
fn bool_from_env_var (env_var_name : & str) -> Result < Option < bool > > { env :: var (env_var_name) . ok () . map (| value | match value . to_lowercase () . as_str () { "true" | "on" | "1" => Ok (true) , "false" | "off" | "0" => Ok (false) , _ => bail ! ("{} must be 'true', 'on', '1', 'false', 'off' or '0'." , env_var_name) , }) . transpose () }
};
}
