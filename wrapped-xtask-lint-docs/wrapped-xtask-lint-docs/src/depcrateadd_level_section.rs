// Generated macro for add_level_section (function)
macro_rules! Depcrateadd_level_section {
() => {
// Module: crate
// Provides: {"add_level_section"}
// Dependencies: {}
fn add_level_section (level : LintLevel , lint_names : & [& str] , buf : & mut String) -> std :: fmt :: Result { let title = match level { LintLevel :: Allow => "Allowed-by-default" , LintLevel :: Warn => "Warn-by-default" , LintLevel :: Deny => "Deny-by-default" , LintLevel :: Forbid => "Forbid-by-default" , } ; writeln ! (buf , "## {title}\n") ? ; writeln ! (buf , "These lints are all set to the '{}' level by default." , level) ? ; for name in lint_names { writeln ! (buf , "- [`{}`](#{})" , name , name) ? ; } writeln ! (buf) ? ; Ok (()) }
};
}
