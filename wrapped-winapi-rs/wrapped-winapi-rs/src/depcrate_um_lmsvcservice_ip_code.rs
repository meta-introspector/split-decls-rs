// Generated macro for SERVICE_IP_CODE (function)
macro_rules! Depcrate_um_lmsvcSERVICE_IP_CODE {
() => {
// Module: crate::um::lmsvc
// Provides: {"SERVICE_IP_CODE"}
// Dependencies: {}
# [inline] pub fn SERVICE_IP_CODE (tt : DWORD , nn : DWORD) -> c_long { (SERVICE_IP_QUERY_HINT | (nn | (tt << SERVICE_IP_WAITTIME_SHIFT))) as c_long }
};
}
