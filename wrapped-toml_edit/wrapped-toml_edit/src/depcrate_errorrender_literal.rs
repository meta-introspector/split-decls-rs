// Generated macro for render_literal (function)
macro_rules! Depcrate_errorrender_literal {
() => {
// Module: crate::error
// Provides: {"render_literal"}
// Dependencies: {}
fn render_literal (literal : & str) -> String { match literal { "\n" => "newline" . to_owned () , "`" => "'`'" . to_owned () , s if s . chars () . all (| c | c . is_ascii_control ()) => { format ! ("`{}`" , s . escape_debug ()) } s => format ! ("`{s}`") , } }
};
}
