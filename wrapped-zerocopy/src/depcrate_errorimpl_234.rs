// Generated macro for impl_234 (impl)
macro_rules! Depcrate_errorimpl_234 {
() => {
// Module: crate::error
// Provides: {"impl_234"}
// Dependencies: {}
# [cfg (any (zerocopy_core_error_1_81_0 , feature = "std" , test))] # [cfg_attr (doc_cfg , doc (cfg (all (rust = "1.81.0" , feature = "std"))))] impl < Src , Dst : ? Sized > Error for ValidityError < Src , Dst > where Dst : KnownLayout + TryFromBytes { }
};
}
