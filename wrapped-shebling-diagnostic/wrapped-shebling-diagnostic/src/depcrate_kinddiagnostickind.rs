// Generated macro for DiagnosticKind (enum)
macro_rules! Depcrate_kindDiagnosticKind {
() => {
// Module: crate::kind
// Provides: {"DiagnosticKind"}
// Dependencies: {}
# [doc = " Diagnostic kinds reported by `shebling`."] # [derive (Debug , miette :: Diagnostic , thiserror :: Error)] # [decl (enum , name = "DiagnosticKind" , vis = "pub" , hash = "6e3ea660")] pub enum DiagnosticKind { # [error ("bad escaping!")] # [diagnostic (code (shebling :: bad_escape))] BadEscape , # [error ("bad operator!")] # [diagnostic (code (shebling :: bad_operator))] BadOperator , # [error ("bad quote!")] # [diagnostic (code (shebling :: bad_quote))] BadQuote , # [error ("literal carriage return")] # [diagnostic (code (shebling :: carriage_return))] # [help ("try running the script through tr -d '\\r'")] CarrigeReturn , # [error ("unicode character!")] # [diagnostic (code (shebling :: unichar) , help ("Delete and retype it, or quote it if intended."))] Unichar , }
};
}
