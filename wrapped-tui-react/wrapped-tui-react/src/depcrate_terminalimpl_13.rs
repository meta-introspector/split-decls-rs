// Generated macro for impl_13 (impl)
macro_rules! Depcrate_terminalimpl_13 {
() => {
// Module: crate::terminal
// Provides: {"impl_13"}
// Dependencies: {}
impl < B > Drop for Terminal < B > where B : Backend , { fn drop (& mut self) { if self . hidden_cursor { if let Err (err) = self . show_cursor () { error ! ("Failed to show the cursor: {}" , err) ; } } } }
};
}
