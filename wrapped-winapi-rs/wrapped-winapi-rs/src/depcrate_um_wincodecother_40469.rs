// Generated macro for other_40469 (other)
macro_rules! Depcrate_um_wincodecother_40469 {
() => {
// Module: crate::um::wincodec
// Provides: {"other_40469"}
// Dependencies: {}
extern "system" { pub fn WICConvertBitmapSource (dstFormat : REFWICPixelFormatGUID , pISrc : * const IWICBitmapSource , ppIDst : * mut * mut IWICBitmapSource ,) -> HRESULT ; pub fn WICCreateBitmapFromSection (width : UINT , height : UINT , pixelFormat : REFWICPixelFormatGUID , hSection : HANDLE , stride : UINT , offset : UINT , ppIBitmap : * mut * mut IWICBitmap ,) -> HRESULT ; pub fn WICCreateBitmapFromSectionEx (width : UINT , height : UINT , pixelFormat : REFWICPixelFormatGUID , hSection : HANDLE , stride : UINT , offset : UINT , desiredAccessLevel : WICSectionAccessLevel , ppIBitmap : * mut * mut IWICBitmap ,) -> HRESULT ; pub fn WICMapGuidToShortName (guid : REFGUID , cchName : UINT , wzName : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT ; pub fn WICMapShortNameToGuid (wzName : PCWSTR , pguid : * mut GUID ,) -> HRESULT ; pub fn WICMapSchemaToName (guidMetadataFormat : REFGUID , pwzSchema : LPWSTR , cchName : UINT , wzName : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT ; }
};
}
