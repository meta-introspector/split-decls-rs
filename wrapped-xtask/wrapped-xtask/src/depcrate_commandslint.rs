// Generated macro for lint (function)
macro_rules! Depcrate_commandslint {
() => {
// Module: crate::commands
// Provides: {"lint"}
// Dependencies: {}
# [doc = " Lint formatting, typos, clippy, and docs (and a soft fail on markdown)"] fn lint () -> Result < () > { Clippy { fix : false } . run () ? ; Docs { open : false } . run () ? ; Format { check : true } . run () ? ; Typos { fix : false } . run () ? ; if let Err (err) = lint_markdown () { tracing :: warn ! ("known issue: markdownlint is currently noisy and can be ignored: {err}") ; } Ok (()) }
};
}
