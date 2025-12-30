// Generated macro for parse_test_config (function)
macro_rules! Depcrate_configparse_test_config {
() => {
// Module: crate::config
// Provides: {"parse_test_config"}
// Dependencies: {}
# [doc = " Parses the configuration `T` from `contents` in comments at the start of the"] # [doc = " file where comments are lines prefixed by `comment`."] pub fn parse_test_config < T > (contents : & str , comment : & str) -> Result < T > where T : DeserializeOwned , { let config_lines : Vec < _ > = contents . lines () . take_while (| l | l . starts_with (comment)) . map (| l | & l [comment . len () ..]) . collect () ; let config_text = config_lines . join ("\n") ; toml :: from_str (& config_text) . context ("failed to parse the test configuration") }
};
}
