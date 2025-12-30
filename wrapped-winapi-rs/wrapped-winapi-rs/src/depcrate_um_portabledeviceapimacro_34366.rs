// Generated macro for macro_34366 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34366 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34366"}
// Dependencies: {}
RIDL ! { # [uuid (0x6a96ed84 , 0x7c73 , 0x4480 , 0x99 , 0x38 , 0xbf , 0x5a , 0xf4 , 0x77 , 0xd4 , 0x26)] interface IPortableDeviceContent (IPortableDeviceContentVtbl) : IUnknown (IUnknownVtbl) { fn EnumObjects (dwFlags : DWORD , pszParentObjectID : LPCWSTR , pFilter : * mut IPortableDeviceValues , ppEnum : * mut * mut IEnumPortableDeviceObjectIDs ,) -> HRESULT , fn Properties (ppProperties : * mut * mut IPortableDeviceProperties ,) -> HRESULT , fn Transfer (ppResources : * mut * mut IPortableDeviceResources ,) -> HRESULT , fn CreateObjectWithPropertiesOnly (pValues : * mut IPortableDeviceValues , ppszObjectID : * mut LPWSTR ,) -> HRESULT , fn CreateObjectWithPropertiesAndData (pValues : * mut IPortableDeviceValues , ppData : * mut * mut IStream , pdwOptimalWriteBufferSize : * mut DWORD , ppszCookie : * mut LPWSTR ,) -> HRESULT , fn Delete (dwOptions : DWORD , pObjectIDs : * mut IPortableDevicePropVariantCollection , ppResults : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetObjectIDsFromPersistentUniqueIDs (pPersistentUniqueIDs : * mut IPortableDevicePropVariantCollection , ppObjectIDs : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn Cancel () -> HRESULT , fn Move (pObjectIDs : * mut IPortableDevicePropVariantCollection , pszDestinationFolderObjectID : LPCWSTR , ppResults : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn Copy (pObjectIDs : * mut IPortableDevicePropVariantCollection , pszDestinationFolderObjectID : LPCWSTR , ppResults : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , } }
};
}
