// Generated macro for impl_17 (impl)
macro_rules! Depcrate_printer_prettyimpl_17 {
() => {
// Module: crate::printer::pretty
// Provides: {"impl_17"}
// Dependencies: {}
impl Formatter for Pretty { type Error = fmt :: Error ; fn fmt (& self , tree : & Tree) -> Result < String , fmt :: Error > { let mut writer = String :: with_capacity (256) ; Pretty :: format_tree (tree , None , & mut IndentVec :: new () , & mut writer) ? ; Ok (writer) } }
};
}
