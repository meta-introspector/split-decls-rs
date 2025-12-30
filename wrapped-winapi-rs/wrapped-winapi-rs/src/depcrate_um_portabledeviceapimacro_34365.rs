// Generated macro for macro_34365 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34365 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34365"}
// Dependencies: {}
RIDL ! { # [uuid (0x625e2df8 , 0x6392 , 0x4cf0 , 0x9a , 0xd1 , 0x3c , 0xfa , 0x5f , 0x17 , 0x77 , 0x5c)] interface IPortableDevice (IPortableDeviceVtbl) : IUnknown (IUnknownVtbl) { fn Open (pszPnPDeviceID : LPCWSTR , pClientInfo : * mut IPortableDeviceValues ,) -> HRESULT , fn SendCommand (dwFlags : DWORD , pParameters : * mut IPortableDeviceValues , ppResults : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn Content (ppContent : * mut * mut IPortableDeviceContent ,) -> HRESULT , fn Capabilities (ppCapabilities : * mut * mut IPortableDeviceCapabilities ,) -> HRESULT , fn Cancel () -> HRESULT , fn Close () -> HRESULT , fn Advise (dwFlags : DWORD , pCallback : * mut IPortableDeviceEventCallback , pParameters : * mut IPortableDeviceValues , ppszCookie : * mut LPWSTR ,) -> HRESULT , fn Unadvise (pszCookie : LPCWSTR ,) -> HRESULT , fn GetPnPDeviceID (ppszPnPDeviceID : * mut LPWSTR ,) -> HRESULT , } }
};
}
