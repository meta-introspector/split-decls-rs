// Generated macro for other_44366 (other)
macro_rules! Depcrate_um_winevtother_44366 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44366"}
// Dependencies: {}
extern "system" { pub fn EvtGetQueryInfo (QueryOrSubscription : EVT_HANDLE , PropertyId : EVT_QUERY_PROPERTY_ID , PropertyValueBufferSize : DWORD , PropertyValueBuffer : PEVT_VARIANT , PropertyValueBufferUsed : PDWORD ,) -> BOOL ; pub fn EvtCreateBookmark (BookmarkXml : LPCWSTR ,) -> EVT_HANDLE ; pub fn EvtUpdateBookmark (Bookmark : EVT_HANDLE , Event : EVT_HANDLE ,) -> BOOL ; pub fn EvtGetEventInfo (Event : EVT_HANDLE , PropertyId : EVT_EVENT_PROPERTY_ID , PropertyValueBufferSize : DWORD , PropertyValueBuffer : PEVT_VARIANT , PropertyValueBufferUsed : PDWORD ,) -> BOOL ; }
};
}
