// Generated macro for try_read_config_file (function)
macro_rules! Depcrate_configtry_read_config_file {
() => {
// Module: crate::config
// Provides: {"try_read_config_file"}
// Dependencies: {}
pub fn try_read_config_file < T : DeserializeOwned > (path : & Path) -> Result < Option < T > > { debug ! ("Attempting to read config file at {:?}" , path) ; let mut file = match File :: open (path) { Ok (f) => f , Err (e) => { debug ! ("Couldn't open config file: {}" , e) ; return Ok (None) ; } } ; let mut string = String :: new () ; match file . read_to_string (& mut string) { Ok (_) => () , Err (e) => { warn ! ("Failed to read config file: {}" , e) ; return Ok (None) ; } } let res = if path . extension () . is_some_and (| e | e == "json") { serde_json :: from_str (& string) . with_context (| | format ! ("Failed to load json config file from {}" , path . display ())) ? } else { toml :: from_str (& string) . with_context (| | format ! ("Failed to load toml config file from {}" , path . display ())) ? } ; Ok (Some (res)) }
};
}
