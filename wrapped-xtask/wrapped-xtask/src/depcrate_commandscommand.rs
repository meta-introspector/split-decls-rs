// Generated macro for Command (enum)
macro_rules! Depcrate_commandsCommand {
() => {
// Module: crate::commands
// Provides: {"Command"}
// Dependencies: {}
# [derive (Clone , Debug , Subcommand)] pub enum Command { # [doc = " Run CI checks (lint, build, test)"] CI , # [doc = " Lint formatting, typos, clippy, and docs"] # [command (visible_alias = "l")] Lint , # [doc = " Build the project"] # [command (visible_alias = "b")] Build , # [command (visible_alias = "c")] Check (Check) , # [doc = " Run tests"] # [command (visible_alias = "t")] Test , # [doc = " Check backend"] # [command (visible_alias = "cb")] CheckBackend (CheckBackend) , # [doc = " Check if README.md is up-to-date (using cargo-rdme)"] # [command (visible_alias = "cr" , alias = "rdme")] Readme (Readme) , # [doc = " Generate code coverage report"] # [command (visible_alias = "cov")] Coverage (Coverage) , # [doc = " Run clippy on the project"] # [command (visible_alias = "cl")] Clippy (Clippy) , # [doc = " Check documentation for errors and warnings"] # [command (name = "docs" , visible_alias = "d")] Docs (Docs) , # [doc = " Check for formatting issues in the project"] # [command (visible_aliases = ["fmt" , "f"])] Format (Format) , # [doc = " Lint markdown files"] # [command (visible_alias = "md")] LintMarkdown , # [doc = " Check for typos in the project"] # [command (visible_alias = "ty")] Typos (Typos) , # [doc = " Test backend"] # [command (visible_alias = "tb")] TestBackend (TestBackend) , # [doc = " Run doc tests"] # [command (visible_alias = "td")] TestDocs , # [doc = " Run lib tests"] # [command (visible_alias = "tl")] TestLibs , # [doc = " Run cargo hack to test each feature in isolation"] # [command (visible_alias = "h")] Hack , }
};
}
