// Generated macro for flush_in (function)
macro_rules! Depcrate_subnormalsflush_in {
() => {
// Module: crate::subnormals
// Provides: {"flush_in"}
// Dependencies: {}
# [cfg (not (all (any (target_arch = "powerpc" , target_arch = "powerpc64") , target_feature = "altivec")))] pub fn flush_in < T : FlushSubnormals > (x : T) -> T { x . flush () }
};
}
