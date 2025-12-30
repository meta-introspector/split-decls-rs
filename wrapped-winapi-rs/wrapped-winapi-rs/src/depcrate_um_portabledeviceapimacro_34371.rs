// Generated macro for macro_34371 (macro)
macro_rules! Depcrate_um_portabledeviceapimacro_34371 {
() => {
// Module: crate::um::portabledeviceapi
// Provides: {"macro_34371"}
// Dependencies: {}
RIDL ! { # [uuid (0x2c8c6dbf , 0xe3dc , 0x4061 , 0xbe , 0xcc , 0x85 , 0x42 , 0xe8 , 0x10 , 0xd1 , 0x26)] interface IPortableDeviceCapabilities (IPortableDeviceCapabilitiesVtbl) : IUnknown (IUnknownVtbl) { fn GetSupportedCommands (ppCommands : * mut * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn GetCommandOptions (Command : REFPROPERTYKEY , ppOptions : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn GetFunctionalCategories (ppCategories : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetFunctionalObjects (Category : REFGUID , ppObjectIDs : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetSupportedContentTypes (Category : REFGUID , ppContentTypes : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetSupportedFormats (ContentType : REFGUID , ppFormats : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetSupportedFormatProperties (Format : REFGUID , ppKeys : * mut * mut IPortableDeviceKeyCollection ,) -> HRESULT , fn GetFixedPropertyAttributes (Format : REFGUID , Key : REFPROPERTYKEY , ppAttributes : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn Cancel () -> HRESULT , fn GetSupportedEvents (ppEvents : * mut * mut IPortableDevicePropVariantCollection ,) -> HRESULT , fn GetEventOptions (Event : REFGUID , ppOptions : * mut * mut IPortableDeviceValues ,) -> HRESULT , } }
};
}
