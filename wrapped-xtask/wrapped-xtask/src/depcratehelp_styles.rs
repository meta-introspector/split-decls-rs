// Generated macro for HELP_STYLES (const)
macro_rules! DepcrateHELP_STYLES {
() => {
// Module: crate
// Provides: {"HELP_STYLES"}
// Dependencies: {}
# [doc = " Matches the clap styling"] pub const HELP_STYLES : Styles = Styles :: styled () . header (AnsiColor :: Green . on_default () . bold ()) . usage (AnsiColor :: Green . on_default () . bold ()) . literal (AnsiColor :: Cyan . on_default () . bold ()) . placeholder (AnsiColor :: Cyan . on_default ()) . error (AnsiColor :: Red . on_default () . bold ()) . valid (AnsiColor :: Cyan . on_default () . bold ()) . invalid (AnsiColor :: Yellow . on_default () . bold ()) ;
};
}
