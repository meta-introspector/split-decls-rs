// Generated macro for LPWSAOVERLAPPED_COMPLETION_ROUTINE (type)
macro_rules! Depcrate_b_dependsLPWSAOVERLAPPED_COMPLETION_ROUTINE {
() => {
// Module: crate::b_depends
// Provides: {"LPWSAOVERLAPPED_COMPLETION_ROUTINE"}
// Dependencies: {}
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option < unsafe extern "system" fn (dwerror : u32 , cbtransferred : u32 , lpoverlapped : * mut OVERLAPPED , dwflags : u32 ,) , > ;
};
}
