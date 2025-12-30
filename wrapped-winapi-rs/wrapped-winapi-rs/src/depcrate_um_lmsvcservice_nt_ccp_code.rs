// Generated macro for SERVICE_NT_CCP_CODE (function)
macro_rules! Depcrate_um_lmsvcSERVICE_NT_CCP_CODE {
() => {
// Module: crate::um::lmsvc
// Provides: {"SERVICE_NT_CCP_CODE"}
// Dependencies: {}
# [inline] pub fn SERVICE_NT_CCP_CODE (tt : DWORD , nn : DWORD) -> c_long { (SERVICE_CCP_QUERY_HINT | nn | ((tt & LOWER_HINT_MASK) << SERVICE_IP_WAITTIME_SHIFT) | ((tt & UPPER_HINT_MASK) << SERVICE_NTIP_WAITTIME_SHIFT)) as c_long }
};
}
