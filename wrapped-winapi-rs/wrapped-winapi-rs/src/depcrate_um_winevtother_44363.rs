// Generated macro for other_44363 (other)
macro_rules! Depcrate_um_winevtother_44363 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44363"}
// Dependencies: {}
extern "system" { pub fn EvtGetObjectArraySize (ObjectArray : EVT_OBJECT_ARRAY_PROPERTY_HANDLE , ObjectArraySize : PDWORD ,) -> BOOL ; pub fn EvtGetObjectArrayProperty (ObjectArray : EVT_OBJECT_ARRAY_PROPERTY_HANDLE , PropertyId : DWORD , ArrayIndex : DWORD , Flags : DWORD , PropertyValueBufferSize : DWORD , PropertyValueBuffer : PEVT_VARIANT , PropertyValueBufferUsed : PDWORD ,) -> BOOL ; }
};
}
