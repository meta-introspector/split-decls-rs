// Generated macro for add_lint (function)
macro_rules! Depcrateadd_lint {
() => {
// Module: crate
// Provides: {"add_lint"}
// Dependencies: {}
fn add_lint (lint : & Lint , buf : & mut String) -> std :: fmt :: Result { writeln ! (buf , "## `{}`" , lint . name) ? ; writeln ! (buf , "Set to `{}` by default" , lint . default_level) ? ; writeln ! (buf , "{}\n" , lint . docs . as_ref () . unwrap ()) }
};
}
