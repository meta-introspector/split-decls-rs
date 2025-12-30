// Generated macro for other_44359 (other)
macro_rules! Depcrate_um_winevtother_44359 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44359"}
// Dependencies: {}
extern "system" { pub fn EvtOpenPublisherEnum (Session : EVT_HANDLE , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtNextPublisherId (PublisherEnum : EVT_HANDLE , PublisherIdBufferSize : DWORD , PublisherIdBuffer : LPWSTR , PublisherIdBufferUsed : PDWORD ,) -> BOOL ; pub fn EvtOpenPublisherMetadata (Session : EVT_HANDLE , PublisherId : LPCWSTR , LogFilePath : LPCWSTR , Locale : LCID , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtGetPublisherMetadataProperty (PublisherMetadata : EVT_HANDLE , PropertyId : EVT_PUBLISHER_METADATA_PROPERTY_ID , Flags : DWORD , PublisherMetadataPropertyBufferSize : DWORD , PublisherMetadataPropertyBuffer : PEVT_VARIANT , PublisherMetadataPropertyBufferUsed : PDWORD ,) -> BOOL ; }
};
}
