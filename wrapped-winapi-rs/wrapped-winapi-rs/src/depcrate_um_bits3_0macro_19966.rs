// Generated macro for macro_19966 (macro)
macro_rules! Depcrate_um_bits3_0macro_19966 {
() => {
// Module: crate::um::bits3_0
// Provides: {"macro_19966"}
// Dependencies: {}
RIDL ! { # [uuid (0x659cdead , 0x489e , 0x11d9 , 0xa9 , 0xcd , 0x00 , 0x0d , 0x56 , 0x96 , 0x52 , 0x51)] interface IBitsPeerCacheAdministration (IBitsPeerCacheAdministrationVtbl) : IUnknown (IUnknownVtbl) { fn GetMaximumCacheSize (pBytes : * mut DWORD ,) -> HRESULT , fn SetMaximumCacheSize (Bytes : DWORD ,) -> HRESULT , fn GetMaximumContentAge (pSeconds : * mut ULONG ,) -> HRESULT , fn SetMaximumContentAge (Seconds : ULONG ,) -> HRESULT , fn GetConfigurationFlags (pFlags : * mut DWORD ,) -> HRESULT , fn SetConfigurationFlags (Flags : DWORD ,) -> HRESULT , fn EnumRecords (ppEnum : * mut * mut IEnumBitsPeerCacheRecords ,) -> HRESULT , fn GetRecord (ppRecord : * mut * mut IBitsPeerCacheRecord ,) -> HRESULT , fn ClearRecords () -> HRESULT , fn DeleteRecord (id : REFGUID ,) -> HRESULT , fn DeleteUrl (url : LPCWSTR ,) -> HRESULT , fn EnumPeers (ppEnum : * mut * mut IEnumBitsPeers ,) -> HRESULT , fn ClearPeers () -> HRESULT , fn DiscoverPeers () -> HRESULT , } }
};
}
