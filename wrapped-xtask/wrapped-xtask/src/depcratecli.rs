// Generated macro for Cli (enum)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser)] enum Cli { Build (build :: Build) , # [command (subcommand)] Ci (ci :: Ci) , Clippy (clippy :: Clippy) , Doc (doc :: Doc) , }
};
}
