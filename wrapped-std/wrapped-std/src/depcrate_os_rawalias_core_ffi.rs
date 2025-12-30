// Generated macro for alias_core_ffi (macro)
macro_rules! Depcrate_os_rawalias_core_ffi {
() => {
// Module: crate::os::raw
// Provides: {"alias_core_ffi"}
// Dependencies: {}
macro_rules ! alias_core_ffi { ($ ($ t : ident) *) => { $ (# [stable (feature = "raw_os" , since = "1.1.0")] # [doc = include_str ! (concat ! ("../../../../core/src/ffi/" , stringify ! ($ t) , ".md"))] # [doc (cfg (all ()))] pub type $ t = core :: ffi ::$ t ;) * } }
};
}
