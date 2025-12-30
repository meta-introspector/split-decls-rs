// Generated macro for macro_40462 (macro)
macro_rules! Depcrate_um_wincodecmacro_40462 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40462"}
// Dependencies: {}
RIDL ! { # [uuid (0xe87a44c4 , 0xb76e , 0x4c47 , 0x8b , 0x09 , 0x29 , 0x8e , 0xb1 , 0x2a , 0x27 , 0x14)] interface IWICBitmapCodecInfo (IWICBitmapCodecInfoVtbl) : IWICComponentInfo (IWICComponentInfoVtbl) { fn GetContainerFormat (pguidContainerFormat : * mut GUID ,) -> HRESULT , fn GetPixelFormats (cFormats : UINT , pguidPixelFormats : * mut GUID , pcActual : * mut UINT ,) -> HRESULT , fn GetColorManagementVersion (cchColorManagementVersion : UINT , wzColorManagementVersion : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetDeviceManufacturer (cchDeviceManufacturer : UINT , wzDeviceManufacturer : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetDeviceModels (cchDeviceModels : UINT , wzDeviceModels : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetMimeTypes (cchMimeTypes : UINT , wzMimeTypes : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetFileExtensions (cchFileExtensions : UINT , wzFileExtensions : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn DoesSupportAnimation (pfSupportAnimation : * mut BOOL ,) -> HRESULT , fn DoesSupportChromakey (pfSupportChromakey : * mut BOOL ,) -> HRESULT , fn DoesSupportLossless (pfSupportLossless : * mut BOOL ,) -> HRESULT , fn DoesSupportMultiframe (pfSupportMultiframe : * mut BOOL ,) -> HRESULT , fn MatchesMimeType (wzMimeType : LPCWSTR , pfMatches : * mut BOOL ,) -> HRESULT , } }
};
}
