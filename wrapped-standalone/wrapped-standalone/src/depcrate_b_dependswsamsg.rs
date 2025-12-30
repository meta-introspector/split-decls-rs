// Generated macro for WSAMSG (struct)
macro_rules! Depcrate_b_dependsWSAMSG {
() => {
// Module: crate::b_depends
// Provides: {"WSAMSG"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy)] pub struct WSAMSG { pub name : * mut SOCKADDR , pub namelen : i32 , pub lpBuffers : * mut WSABUF , pub dwBufferCount : u32 , pub Control : WSABUF , pub dwFlags : u32 , }
};
}
