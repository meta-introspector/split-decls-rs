// Generated macro for macro_34368 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34368 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34368"}
// Dependencies: {}
RIDL ! { # [uuid (0x10ece955 , 0xcf41 , 0x4728 , 0xbf , 0xa0 , 0x41 , 0xee , 0xdf , 0x1b , 0xbf , 0x19)] interface IEnumPortableDeviceObjectIDs (IEnumPortableDeviceObjectIDsVtbl) : IUnknown (IUnknownVtbl) { fn Next (cObjects : ULONG , pObjIDs : * mut LPWSTR , pcFetched : * mut ULONG ,) -> HRESULT , fn Skip (cObjects : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppEnum : * mut * mut IEnumPortableDeviceObjectIDs ,) -> HRESULT , fn Cancel () -> HRESULT , } }
};
}
