// Generated macro for macro_2 (macro)
macro_rules! Depcrate_bindingsmacro_2 {
() => {
// Module: crate::bindings
// Provides: {"macro_2"}
// Dependencies: {}
windows_link :: link ! ("advapi32.dll" "system" fn RegGetValueA (hkey : HKEY , lpsubkey : PCSTR , lpvalue : PCSTR , dwflags : REG_ROUTINE_FLAGS , pdwtype : * mut REG_VALUE_TYPE , pvdata : * mut core :: ffi :: c_void , pcbdata : * mut u32) -> WIN32_ERROR) ;
};
}
