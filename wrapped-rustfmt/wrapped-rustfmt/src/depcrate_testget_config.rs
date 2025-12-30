// Generated macro for get_config (function)
macro_rules! Depcrate_testget_config {
() => {
// Module: crate::test
// Provides: {"get_config"}
// Dependencies: {}
fn get_config (config_file : Option < & Path > , edition : Option < Edition > , style_edition : Option < StyleEdition > , version : Option < Version > ,) -> Config { let config_file_name = match config_file { None => return Config :: default_for_possible_style_edition (style_edition , edition , version) , Some (file_name) => { let mut full_path = PathBuf :: from ("tests/config/") ; full_path . push (file_name) ; if ! full_path . exists () { return Config :: default_for_possible_style_edition (style_edition , edition , version) ; } ; full_path } } ; let mut def_config_file = fs :: File :: open (config_file_name) . expect ("couldn't open config") ; let mut def_config = String :: new () ; def_config_file . read_to_string (& mut def_config) . expect ("Couldn't read config") ; Config :: from_toml_for_style_edition (& def_config , Path :: new ("tests/config/") , edition , style_edition , version ,) . expect ("invalid TOML") }
};
}
