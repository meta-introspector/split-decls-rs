// Generated macro for lint_markdown (function)
macro_rules! Depcrate_commandslint_markdown {
() => {
// Module: crate::commands
// Provides: {"lint_markdown"}
// Dependencies: {}
# [doc = " Lint markdown files using [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2)"] fn lint_markdown () -> Result < () > { cmd ! ("markdownlint-cli2" , "**/*.md" , "!target") . run_with_trace () ? ; Ok (()) }
};
}
