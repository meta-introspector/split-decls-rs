// Generated macro for feature (macro)
macro_rules! Depcrate_macrosfeature {
() => {
// Module: crate::macros
// Provides: {"feature"}
// Dependencies: {}
macro_rules ! feature { (#! [$ meta : meta] $ ($ item : item) *) => { $ (# [cfg ($ meta)] # [cfg_attr (docsrs , doc (cfg ($ meta)))] $ item) * } }
};
}
