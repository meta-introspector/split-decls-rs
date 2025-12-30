// Generated macro for peek_eq_op (function)
macro_rules! Depcrate_helperspeek_eq_op {
() => {
// Module: crate::helpers
// Provides: {"peek_eq_op"}
// Dependencies: {}
fn peek_eq_op (input : ParseStream) -> bool { if let Some ((p , _)) = input . cursor () . punct () { p . as_char () == '=' && p . spacing () == Spacing :: Alone } else { false } }
};
}
