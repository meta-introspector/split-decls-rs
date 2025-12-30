// Generated macro for BraceStyle (enum)
macro_rules! Depcrate_config_optionsBraceStyle {
() => {
// Module: crate::config::options
// Provides: {"BraceStyle"}
// Dependencies: {}
# [config_type] # [doc = " Where to put the opening brace of items (`fn`, `impl`, etc.)."] pub enum BraceStyle { # [doc = " Put the opening brace on the next line."] AlwaysNextLine , # [doc = " Put the opening brace on the same line, if possible."] PreferSameLine , # [doc = " Prefer the same line except where there is a where-clause, in which"] # [doc = " case force the brace to be put on the next line."] SameLineWhere , }
};
}
