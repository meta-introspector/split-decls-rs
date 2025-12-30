// Generated macro for WSASENDMSG (struct)
macro_rules! Depcrate_b_dependsWSASENDMSG {
() => {
// Module: crate::b_depends
// Provides: {"WSASENDMSG"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct WSASENDMSG { pub lpMsg : * mut WSAMSG , pub dwFlags : u32 , pub lpNumberOfBytesSent : * mut u32 , pub lpOverlapped : * mut OVERLAPPED , pub lpCompletionRoutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE , }
};
}
