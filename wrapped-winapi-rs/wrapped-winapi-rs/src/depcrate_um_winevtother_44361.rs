// Generated macro for other_44361 (other)
macro_rules! Depcrate_um_winevtother_44361 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44361"}
// Dependencies: {}
extern "system" { pub fn EvtOpenEventMetadataEnum (PublisherMetadata : EVT_HANDLE , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtNextEventMetadata (EventMetadataEnum : EVT_HANDLE , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtGetEventMetadataProperty (EventMetadata : EVT_HANDLE , PropertyId : EVT_EVENT_METADATA_PROPERTY_ID , Flags : DWORD , EventMetadataPropertyBufferSize : DWORD , EventMetadataPropertyBuffer : PEVT_VARIANT , EventMetadataPropertyBufferUsed : PDWORD ,) -> BOOL ; }
};
}
