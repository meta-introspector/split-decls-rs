// Generated macro for cfg_codec (macro)
macro_rules! Depcrate_cfgcfg_codec {
() => {
// Module: crate::cfg
// Provides: {"cfg_codec"}
// Dependencies: {}
macro_rules ! cfg_codec { ($ ($ item : item) *) => { $ (# [cfg (feature = "codec")] # [cfg_attr (docsrs , doc (cfg (feature = "codec")))] $ item) * } }
};
}
