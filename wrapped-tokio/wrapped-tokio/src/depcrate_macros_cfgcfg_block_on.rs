// Generated macro for cfg_block_on (macro)
macro_rules! Depcrate_macros_cfgcfg_block_on {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_block_on"}
// Dependencies: {}
# [doc = " Enables `enter::block_on`."] macro_rules ! cfg_block_on { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "fs" , feature = "net" , feature = "io-std" , feature = "rt" ,))] $ item) * } }
};
}
