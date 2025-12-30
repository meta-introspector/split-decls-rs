// Generated macro for macro_23952 (macro)
macro_rules! Depcrate_um_d2d1_1macro_23952 {
() => {
// Module: crate::um::d2d1_1
// Provides: {"macro_23952"}
// Dependencies: {}
RIDL ! { # [uuid (0x47dd575d , 0xac05 , 0x4cdd , 0x80 , 0x49 , 0x9b , 0x02 , 0xcd , 0x16 , 0xf4 , 0x4c)] interface ID2D1Device (ID2D1DeviceVtbl) : ID2D1Resource (ID2D1ResourceVtbl) { fn CreateDeviceContext (options : D2D1_DEVICE_CONTEXT_OPTIONS , deviceContext : * mut * mut ID2D1DeviceContext ,) -> HRESULT , fn CreatePrintControl (wicFactory : * const IWICImagingFactory , documentTarget : * const IPrintDocumentPackageTarget , printControlProperties : * const D2D1_PRINT_CONTROL_PROPERTIES , printControl : * mut * mut ID2D1PrintControl ,) -> HRESULT , fn SetMaximumTextureMemory (maximumInBytes : UINT64 ,) -> () , fn GetMaximumTextureMemory () -> UINT64 , fn ClearResources (millisecondsSinceUse : UINT32 ,) -> () , } }
};
}
