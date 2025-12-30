// Generated macro for macro_40653 (macro)
macro_rules! Depcrate_um_wincodecsdkmacro_40653 {
() => {
// Module: crate::um::wincodecsdk
// Provides: {"macro_40653"}
// Dependencies: {}
RIDL ! { # [uuid (0xaba958bf , 0xc672 , 0x44d1 , 0x8d , 0x61 , 0xce , 0x6d , 0xf2 , 0xe6 , 0x82 , 0xc2)] interface IWICMetadataHandlerInfo (IWICMetadataHandlerInfoVtbl) : IWICComponentInfo (IWICComponentInfoVtbl) { fn GetMetadataFormat (pguidMetadataFormat : * mut GUID ,) -> HRESULT , fn GetContainerFormats (cContainerFormats : UINT , pguidContainerFormats : * mut GUID , pcchActual : * mut UINT ,) -> HRESULT , fn GetDeviceManufacturer (cchDeviceManufacturer : UINT , wzDeviceManufacturer : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetDeviceModels (cchDeviceModels : UINT , wzDeviceModels : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn DoesRequireFullStream (pfRequiresFullStream : * mut BOOL ,) -> HRESULT , fn DoesSupportPadding (pfSupportsPadding : * mut BOOL ,) -> HRESULT , fn DoesRequireFixedSize (pfFixedSize : * mut BOOL ,) -> HRESULT , } }
};
}
