// Generated macro for macro_3462 (macro)
macro_rules! Depcrate_shared_dxgi1_4macro_3462 {
() => {
// Module: crate::shared::dxgi1_4
// Provides: {"macro_3462"}
// Dependencies: {}
RIDL ! { # [uuid (0x645967a4 , 0x1392 , 0x4310 , 0xa7 , 0x98 , 0x80 , 0x53 , 0xce , 0x3e , 0x93 , 0xfd)] interface IDXGIAdapter3 (IDXGIAdapter3Vtbl) : IDXGIAdapter2 (IDXGIAdapter2Vtbl) { fn RegisterHardwareContentProtectionTeardownStatusEvent (hEvent : HANDLE , pdwCookie : * mut DWORD ,) -> HRESULT , fn UnregisterHardwareContentProtectionTeardownStatus (dwCookie : DWORD ,) -> () , fn QueryVideoMemoryInfo (NodeIndex : UINT , MemorySegmentGroup : DXGI_MEMORY_SEGMENT_GROUP , pVideoMemoryInfo : * mut DXGI_QUERY_VIDEO_MEMORY_INFO ,) -> HRESULT , fn SetVideoMemoryReservation (NodeIndex : UINT , MemorySegmentGroup : DXGI_MEMORY_SEGMENT_GROUP , Reservation : UINT64 ,) -> HRESULT , fn RegisterVideoMemoryBudgetChangeNotificationEvent (hEvent : HANDLE , pdwCookie : * mut DWORD ,) -> HRESULT , fn UnregisterVideoMemoryBudgetChangeNotification (dwCookie : DWORD ,) -> () , } }
};
}
