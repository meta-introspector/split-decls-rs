// Generated macro for HRESULT_FROM_NT (function)
macro_rules! Depcrate_shared_winerrorHRESULT_FROM_NT {
() => {
// Module: crate::shared::winerror
// Provides: {"HRESULT_FROM_NT"}
// Dependencies: {}
# [inline] pub fn HRESULT_FROM_NT (x : c_ulong) -> HRESULT { (x | FACILITY_NT_BIT as u32) as i32 }
};
}
