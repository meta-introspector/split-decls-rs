// Generated macro for other_44339 (other)
macro_rules! Depcrate_um_winevtother_44339 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44339"}
// Dependencies: {}
extern "system" { pub fn EvtSubscribe (Session : EVT_HANDLE , SignalEvent : HANDLE , ChannelPath : LPCWSTR , Query : LPCWSTR , Bookmark : EVT_HANDLE , Context : PVOID , Callback : EVT_SUBSCRIBE_CALLBACK , Flags : DWORD ,) -> EVT_HANDLE ; }
};
}
