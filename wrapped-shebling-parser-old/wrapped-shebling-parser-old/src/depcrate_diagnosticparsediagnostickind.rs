// Generated macro for ParseDiagnosticKind (enum)
macro_rules! Depcrate_diagnosticParseDiagnosticKind {
() => {
// Module: crate::diagnostic
// Provides: {"ParseDiagnosticKind"}
// Dependencies: {}
# [derive (Clone , Debug , Error , miette :: Diagnostic)] pub (crate) enum ParseDiagnosticKind { # [error ("ambiguous command!")] # [diagnostic (code (shebling :: ambiguous))] Ambiguous , # [error ("bad escaping!")] # [diagnostic (code (shebling :: bad_escape))] BadEscape , # [error ("bad operator!")] # [diagnostic (code (shebling :: bad_operator))] BadOperator , # [error ("bad spacing!")] # [diagnostic (code (shebling :: bad_space))] BadSpace , # [error ("you should escape this character.")] # [diagnostic (code (shebling :: missing_escape))] MissingEscape , # [error ("you need a space here.")] # [diagnostic (code (shebling :: missing_space))] MissingSpace , # [error ("incorrect shell syntax!")] # [diagnostic (code (shebling :: not_shell_code))] NotShellCode , # [error ("this code looks a bit suspicious.")] # [diagnostic (code (shebling :: sus_token))] SusToken , # [error ("this assignment's value looks kinda sus.")] # [diagnostic (code (shebling :: sus_value))] SusValue , # [error ("you're missing some curlies here.")] # [diagnostic (code (shebling :: unbraced))] Unbraced , # [error ("unclosed string!")] # [diagnostic (code (shebling :: unclosed_string))] UnclosedString , # [error ("unicode character!")] # [diagnostic (code (shebling :: unichar) , help ("Delete and retype it, or quote it if intended."))] Unichar , }
};
}
