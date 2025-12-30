// Generated macro for test_libs (function)
macro_rules! Depcrate_commandstest_libs {
() => {
// Module: crate::commands
// Provides: {"test_libs"}
// Dependencies: {}
# [doc = " Run lib tests for the workspace's default packages"] fn test_libs () -> Result < () > { run_cargo (vec ! ["hack" , "--ignore-private" , "--exclude" , "ratatui-crossterm" , "test" , "--lib" , "--all-targets" , "--all-features" ,]) ? ; let crossterm_feature = CROSSTERM_COMMON_FEATURES . join (",") ; for crossterm_version in crate :: CROSSTERM_VERSION_FEATURES { let features = format ! ("{crossterm_feature},{crossterm_version}") ; run_cargo (vec ! ["test" , "--package" , "ratatui-crossterm" , "--lib" , "--no-default-features" , "--features" , features . as_str () ,]) ? ; } Ok (()) }
};
}
