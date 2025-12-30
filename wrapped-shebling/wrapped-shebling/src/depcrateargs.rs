// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (clap :: Parser)] # [command (author , version , about , help_template = "{name}
{tab}{about-with-newline}
{tab}{author-with-newline}
{usage-heading} {usage}

{all-args}")] struct Args { # [doc = " Path to the file to lint."] # [arg (short , long)] path : PathBuf , }
};
}
