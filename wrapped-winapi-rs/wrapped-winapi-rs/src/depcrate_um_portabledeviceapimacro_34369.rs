// Generated macro for macro_34369 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34369 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34369"}
// Dependencies: {}
RIDL ! { # [uuid (0x7f6d695c , 0x03df , 0x4439 , 0xa8 , 0x09 , 0x59 , 0x26 , 0x6b , 0xee , 0xe3 , 0xa6)] interface IPortableDeviceProperties (IPortableDevicePropertiesVtbl) : IUnknown (IUnknownVtbl) { fn GetSupportedProperties (pszObjectID : LPCWSTR , ppKeys : * mut * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn GetPropertyAttributes (pszObjectID : LPCWSTR , Key : REFPROPERTYKEY , ppAttributes : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn GetValues (pszObjectID : LPCWSTR , pKeys : * mut IPortableDeviceKeyCollection , ppValues : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn SetValues (pszObjectID : LPCWSTR , pValues : * mut IPortableDeviceValues , ppResults : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn Delete (pszObjectID : LPCWSTR , pKeys : * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn Cancel () -> HRESULT , } }
};
}
