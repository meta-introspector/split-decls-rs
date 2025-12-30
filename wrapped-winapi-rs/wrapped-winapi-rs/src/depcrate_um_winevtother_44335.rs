// Generated macro for other_44335 (other)
macro_rules! Depcrate_um_winevtother_44335 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44335"}
// Dependencies: {}
extern "system" { pub fn EvtQuery (Session : EVT_HANDLE , Path : LPCWSTR , Query : LPCWSTR , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtNext (ResultSet : EVT_HANDLE , EventsSize : DWORD , Events : PEVT_HANDLE , Timeout : DWORD , Flags : DWORD , Returned : PDWORD ,) -> BOOL ; pub fn EvtSeek (ResultSet : EVT_HANDLE , Position : LONGLONG , Bookmark : EVT_HANDLE , Timeout : DWORD , Flags : DWORD ,) -> BOOL ; }
};
}
