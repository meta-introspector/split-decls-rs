// Generated macro for impl_1399 (impl)
macro_rules! Depcrate_test_configuration_snippetimpl_1399 {
() => {
// Module: crate::test::configuration_snippet
// Provides: {"impl_1399"}
// Dependencies: {}
impl ConfigurationSection { fn get_section < I : Iterator < Item = String > > (file : & mut Enumerate < I > ,) -> Option < ConfigurationSection > { let config_name_regex = static_regex ! (r"^## `([^`]+)`") ; let config_value_regex = static_regex ! (r#"^#### `"?([^`]+?)"?`"#) ; loop { match file . next () { Some ((i , line)) => { if line . starts_with ("```rust") { let lines : Vec < String > = file . map (| (_i , l) | l) . take_while (| l | ! l . starts_with ("```")) . collect () ; let block = format ! ("{}\n" , lines . join ("\n")) ; let start_line = (i + 2) as u32 ; return Some (ConfigurationSection :: CodeBlock ((block , start_line))) ; } else if let Some (c) = config_name_regex . captures (& line) { return Some (ConfigurationSection :: ConfigName (String :: from (& c [1]))) ; } else if let Some (c) = config_value_regex . captures (& line) { return Some (ConfigurationSection :: ConfigValue (String :: from (& c [1]))) ; } } None => return None , } } } }
};
}
