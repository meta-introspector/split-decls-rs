// Generated macro for macro_51933 (macro)
macro_rules! Depcrate_um_winntmacro_51933 {
() => {
// Module: crate::um::winnt
// Provides: {"macro_51933"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { STRUCT ! { struct NV_MEMORY_RANGE { BaseAddress : * mut VOID , Length : SIZE_T , } } pub type PNV_MEMORY_RANGE = * mut NV_MEMORY_RANGE ; pub const FLUSH_NV_MEMORY_IN_FLAG_NO_DRAIN : ULONG = 0x00000001 ; pub const FLUSH_NV_MEMORY_DEFAULT_TOKEN : ULONG_PTR = - 1isize as usize ; }
};
}
