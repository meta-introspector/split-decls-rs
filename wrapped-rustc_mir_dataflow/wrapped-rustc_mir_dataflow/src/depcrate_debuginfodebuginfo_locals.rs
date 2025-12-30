// Generated macro for debuginfo_locals (function)
macro_rules! Depcrate_debuginfodebuginfo_locals {
() => {
// Module: crate::debuginfo
// Provides: {"debuginfo_locals"}
// Dependencies: {}
# [doc = " Return the set of locals that appear in debuginfo."] pub fn debuginfo_locals (body : & Body < '_ >) -> DenseBitSet < Local > { let mut visitor = DebuginfoLocals (DenseBitSet :: new_empty (body . local_decls . len ())) ; for debuginfo in body . var_debug_info . iter () { visitor . visit_var_debug_info (debuginfo) ; } visitor . 0 }
};
}
