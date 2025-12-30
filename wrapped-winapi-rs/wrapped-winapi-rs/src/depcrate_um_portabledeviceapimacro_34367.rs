// Generated macro for macro_34367 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34367 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34367"}
// Dependencies: {}
RIDL ! { # [uuid (0x9b4add96 , 0xf6bf , 0x4034 , 0x87 , 0x08 , 0xec , 0xa7 , 0x2b , 0xf1 , 0x05 , 0x54)] interface IPortableDeviceContent2 (IPortableDeviceContent2Vtbl) : IPortableDeviceContent (IPortableDeviceContentVtbl) { fn UpdateObjectWithPropertiesAndData (pszObjectID : LPCWSTR , pProperties : * mut IPortableDeviceValues , ppData : * mut * mut IStream , pdwOptimalWriteBufferSize : * mut DWORD ,) -> HRESULT , } }
};
}
