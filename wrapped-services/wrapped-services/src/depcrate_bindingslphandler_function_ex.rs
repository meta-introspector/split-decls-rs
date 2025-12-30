// Generated macro for LPHANDLER_FUNCTION_EX (type)
macro_rules! Depcrate_bindingsLPHANDLER_FUNCTION_EX {
() => {
// Module: crate::bindings
// Provides: {"LPHANDLER_FUNCTION_EX"}
// Dependencies: {}
pub type LPHANDLER_FUNCTION_EX = Option < unsafe extern "system" fn (dwcontrol : u32 , dweventtype : u32 , lpeventdata : * mut core :: ffi :: c_void , lpcontext : * mut core :: ffi :: c_void ,) -> u32 , > ;
};
}
