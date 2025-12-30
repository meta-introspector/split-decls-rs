// Generated macro for other_44356 (other)
macro_rules! Depcrate_um_winevtother_44356 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44356"}
// Dependencies: {}
extern "system" { pub fn EvtOpenChannelEnum (Session : EVT_HANDLE , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtNextChannelPath (ChannelEnum : EVT_HANDLE , ChannelPathBufferSize : DWORD , ChannelPathBuffer : LPWSTR , ChannelPathBufferUsed : PDWORD ,) -> BOOL ; pub fn EvtOpenChannelConfig (Session : EVT_HANDLE , ChannelPath : LPCWSTR , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtSaveChannelConfig (ChannelConfig : EVT_HANDLE , Flags : DWORD ,) -> BOOL ; pub fn EvtSetChannelConfigProperty (ChannelConfig : EVT_HANDLE , PropertyId : EVT_CHANNEL_CONFIG_PROPERTY_ID , Flags : DWORD , PropertyValue : PEVT_VARIANT ,) -> BOOL ; pub fn EvtGetChannelConfigProperty (ChannelConfig : EVT_HANDLE , PropertyId : EVT_CHANNEL_CONFIG_PROPERTY_ID , Flags : DWORD , PropertyValueBufferSize : DWORD , PropertyValueBuffer : PEVT_VARIANT , PropertyValueBufferUsed : PDWORD ,) -> BOOL ; }
};
}
