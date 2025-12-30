// Generated macro for TestCommand (enum)
macro_rules! DepcrateTestCommand {
() => {
// Module: crate
// Provides: {"TestCommand"}
// Dependencies: {}
# [derive (Debug , Subcommand)] # [allow (clippy :: enum_variant_names)] enum TestCommand { TestAll , TestBackcompat , TestBook , TestCross , TestLintCross , TestHost { # [doc = " Whether to skip running ui tests"] # [arg (long)] skip_ui_tests : bool , } , TestLint , TestUi , # [doc = " Run snapshot tests or optionally overwrite the expected output"] TestSnapshot { # [doc = " Overwrite the expected output instead of comparing it."] # [arg (long)] overwrite : bool , # [doc = " Runs a single snapshot test in Debug mode"] single : Option < Snapshot > , } , }
};
}
