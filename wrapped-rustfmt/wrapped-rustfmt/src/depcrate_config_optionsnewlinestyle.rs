// Generated macro for NewlineStyle (enum)
macro_rules! Depcrate_config_optionsNewlineStyle {
() => {
// Module: crate::config::options
// Provides: {"NewlineStyle"}
// Dependencies: {}
# [config_type] pub enum NewlineStyle { # [doc = " Auto-detect based on the raw source input."] Auto , # [doc = " Force CRLF (`\\r\\n`)."] Windows , # [doc = " Force CR (`\\n`)."] Unix , # [doc = " `\\r\\n` in Windows, `\\n` on other platforms."] Native , }
};
}
