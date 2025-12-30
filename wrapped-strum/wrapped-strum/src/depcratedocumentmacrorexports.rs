// Generated macro for DocumentMacroRexports (macro)
macro_rules! DepcrateDocumentMacroRexports {
() => {
// Module: crate
// Provides: {"DocumentMacroRexports"}
// Dependencies: {}
macro_rules ! DocumentMacroRexports { ($ ($ export : ident) ,+) => { $ (# [cfg (all (docsrs , feature = "derive"))] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub use strum_macros ::$ export ;) + } ; }
};
}
