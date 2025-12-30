// Generated macro for test (module)
macro_rules! Depcrate_config_macro_namestest {
() => {
// Module: crate::config::macro_names
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use std :: str :: FromStr ; # [test] fn macro_names_from_str () { let macro_names = MacroSelectors :: from_str (r#"["foo", "*", "bar"]"#) . unwrap () ; assert_eq ! (macro_names , MacroSelectors ([MacroSelector :: Name (MacroName ("foo" . to_owned ())) , MacroSelector :: All , MacroSelector :: Name (MacroName ("bar" . to_owned ()))] . into_iter () . collect ())) ; } # [test] fn macro_names_display () { let macro_names = MacroSelectors :: from_str (r#"["foo", "*", "bar"]"#) . unwrap () ; assert_eq ! (format ! ("{macro_names}") , "foo, *, bar") ; } }
};
}
