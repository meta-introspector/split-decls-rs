// Generated macro for other_44343 (other)
macro_rules! Depcrate_um_winevtother_44343 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44343"}
// Dependencies: {}
extern "system" { pub fn EvtCreateRenderContext (ValuePathsCount : DWORD , ValuePaths : * mut LPCWSTR , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtRender (Context : EVT_HANDLE , Fragment : EVT_HANDLE , Flags : DWORD , BufferSize : DWORD , Buffer : PVOID , BufferUsed : PDWORD , PropertyCount : PDWORD ,) -> BOOL ; }
};
}
