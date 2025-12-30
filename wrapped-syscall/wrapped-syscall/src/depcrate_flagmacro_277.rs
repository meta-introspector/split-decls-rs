// Generated macro for macro_277 (macro)
macro_rules! Depcrate_flagmacro_277 {
() => {
// Module: crate::flag
// Provides: {"macro_277"}
// Dependencies: {}
bitflags ! { pub struct MunmapFlags : usize { # [doc = " Indicates whether the funmap call must implicitly do an msync, for the changes to"] # [doc = " become visible later."] # [doc = ""] # [doc = " This flag will currently be set if and only if MAP_SHARED | PROT_WRITE are set."] const NEEDS_SYNC = 1 ; } }
};
}
