// Generated macro for MacroArgParser (struct)
macro_rules! Depcrate_macrosMacroArgParser {
() => {
// Module: crate::macros
// Provides: {"MacroArgParser"}
// Dependencies: {}
# [doc = " Parses macro arguments on macro def."] struct MacroArgParser { # [doc = " Either a name of the next metavariable, a separator, or junk."] buf : String , # [doc = " The first token of the current buffer."] start_tok : Token , # [doc = " `true` if we are parsing a metavariable or a repeat."] is_meta_var : bool , # [doc = " The last token parsed."] last_tok : Token , # [doc = " Holds the parsed arguments."] result : Vec < ParsedMacroArg > , }
};
}
