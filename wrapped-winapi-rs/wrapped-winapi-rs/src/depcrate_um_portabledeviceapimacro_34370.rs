// Generated macro for macro_34370 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34370 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34370"}
// Dependencies: {}
RIDL ! { # [uuid (0xfd8878ac , 0xd841 , 0x4d17 , 0x89 , 0x1c , 0xe6 , 0x82 , 0x9c , 0xdb , 0x69 , 0x34)] interface IPortableDeviceResources (IPortableDeviceResourcesVtbl) : IUnknown (IUnknownVtbl) { fn GetSupportedResources (pszObjectID : LPCWSTR , ppKeys : * mut * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn GetResourceAttributes (pszObjectID : LPCWSTR , Key : REFPROPERTYKEY , ppResourceAttributes : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn GetStream (pszObjectID : LPCWSTR , Key : REFPROPERTYKEY , dwMode : DWORD , pdwOptimalBufferSize : * mut DWORD , ppStream : * mut * mut IStream ,) -> HRESULT , fn Delete (pszObjectID : LPCWSTR , pKeys : * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn Cancel () -> HRESULT , fn CreateResource (pResourceAttributes : * mut IPortableDeviceValues , ppData : * mut * mut IStream , pdwOptimalWriteBufferSize : * mut DWORD , ppszCookie : * mut LPWSTR ,) -> HRESULT , } }
};
}
