// Generated macro for impl_401 (impl)
macro_rules! Depcrate_config_listsimpl_401 {
() => {
// Module: crate::config::lists
// Provides: {"impl_401"}
// Dependencies: {}
impl DefinitiveListTactic { pub fn ends_with_newline (& self , indent_style : IndentStyle) -> bool { match indent_style { IndentStyle :: Block => * self != DefinitiveListTactic :: Horizontal , IndentStyle :: Visual => false , } } }
};
}
