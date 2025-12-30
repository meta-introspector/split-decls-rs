// Generated macro for macro_34364 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34364 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34364"}
// Dependencies: {}
RIDL ! { # [uuid (0xa1567595 , 0x4c2f , 0x4574 , 0xa6 , 0xfa , 0xec , 0xef , 0x91 , 0x7b , 0x9a , 0x40)] interface IPortableDeviceManager (IPortableDeviceManagerVtbl) : IUnknown (IUnknownVtbl) { fn GetDevices (pPnPDeviceIDs : * mut LPWSTR , pcPnPDeviceIDs : * mut DWORD ,) -> HRESULT , fn RefreshDeviceList () -> HRESULT , fn GetDeviceFriendlyName (pszPnPDeviceID : LPCWSTR , pDeviceFriendlyName : * mut WCHAR , pcchDeviceFriendlyName : * mut DWORD ,) -> HRESULT , fn GetDeviceDescription (pszPnPDeviceID : LPCWSTR , pDeviceDescription : * mut WCHAR , pcchDeviceDescription : * mut DWORD ,) -> HRESULT , fn GetDeviceManufacturer (pszPnPDeviceID : LPCWSTR , pDeviceManufacturer : * mut WCHAR , pcchDeviceManufacturer : * mut DWORD ,) -> HRESULT , fn GetDeviceProperty (pszPnPDeviceID : LPCWSTR , pszDevicePropertyName : LPCWSTR , pData : * mut BYTE , pcbData : * mut DWORD , pdwType : * mut DWORD ,) -> HRESULT , fn GetPrivateDevices (pPnPDeviceIDs : * mut LPWSTR , pcPnPDeviceIDs : * mut DWORD ,) -> HRESULT , } }
};
}
