// Generated macro for HRESULT_FROM_WIN32 (function)
macro_rules! Depcrate_shared_winerrorHRESULT_FROM_WIN32 {
() => {
// Module: crate::shared::winerror
// Provides: {"HRESULT_FROM_WIN32"}
// Dependencies: {}
# [inline] pub fn HRESULT_FROM_WIN32 (x : c_ulong) -> HRESULT { if x as i32 <= 0 { x as i32 } else { ((x & 0x0000FFFF) | ((FACILITY_WIN32 as u32) << 16) | 0x80000000) as i32 } }
};
}
