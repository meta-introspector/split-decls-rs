// Generated macro for other_44348 (other)
macro_rules! Depcrate_um_winevtother_44348 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44348"}
// Dependencies: {}
extern "system" { pub fn EvtOpenLog (Session : EVT_HANDLE , Path : LPCWSTR , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtGetLogInfo (Log : EVT_HANDLE , PropertyId : EVT_LOG_PROPERTY_ID , PropertyValueBufferSize : DWORD , PropertyValueBuffer : PEVT_VARIANT , PropertyValueBufferUsed : PDWORD ,) -> BOOL ; pub fn EvtClearLog (Session : EVT_HANDLE , ChannelPath : LPCWSTR , TargetFilePath : LPCWSTR , Flags : DWORD ,) -> BOOL ; }
};
}
