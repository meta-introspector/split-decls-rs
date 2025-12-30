// Generated macro for MacroArgKind (enum)
macro_rules! Depcrate_macrosMacroArgKind {
() => {
// Module: crate::macros
// Provides: {"MacroArgKind"}
// Dependencies: {}
# [derive (Debug , Clone)] enum MacroArgKind { # [doc = " e.g., `$x: expr`."] MetaVariable (Symbol , String) , # [doc = " e.g., `$($foo: expr),*`"] Repeat (# [doc = " `()`, `[]` or `{}`."] Delimiter , # [doc = " Inner arguments inside delimiters."] Vec < ParsedMacroArg > , # [doc = " Something after the closing delimiter and the repeat token, if available."] Option < Box < ParsedMacroArg > > , # [doc = " The repeat token. This could be one of `*`, `+` or `?`."] Token ,) , # [doc = " e.g., `[derive(Debug)]`"] Delimited (Delimiter , Vec < ParsedMacroArg >) , # [doc = " A possible separator. e.g., `,` or `;`."] Separator (String , String) , # [doc = " Other random stuff that does not fit to other kinds."] # [doc = " e.g., `== foo` in `($x: expr == foo)`."] Other (String , String) , }
};
}
