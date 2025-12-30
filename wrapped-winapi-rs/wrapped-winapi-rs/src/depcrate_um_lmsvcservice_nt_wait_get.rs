// Generated macro for SERVICE_NT_WAIT_GET (function)
macro_rules! Depcrate_um_lmsvcSERVICE_NT_WAIT_GET {
() => {
// Module: crate::um::lmsvc
// Provides: {"SERVICE_NT_WAIT_GET"}
// Dependencies: {}
# [inline] pub fn SERVICE_NT_WAIT_GET (code : DWORD) -> DWORD { ((code & UPPER_GET_HINT_MASK) >> SERVICE_NTIP_WAITTIME_SHIFT) | ((code & LOWER_GET_HINT_MASK) >> SERVICE_IP_WAITTIME_SHIFT) }
};
}
