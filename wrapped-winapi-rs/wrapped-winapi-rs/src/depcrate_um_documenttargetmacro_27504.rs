// Generated macro for macro_27504 (macro)
macro_rules! Depcrate_um_documenttargetmacro_27504 {
() => {
// Module: crate::um::documenttarget
// Provides: {"macro_27504"}
// Dependencies: {}
RIDL ! { # [uuid (0x1b8efec4 , 0x3019 , 0x4c27 , 0x96 , 0x4e , 0x36 , 0x72 , 0x02 , 0x15 , 0x69 , 0x06)] interface IPrintDocumentPackageTarget (IPrintDocumentPackageTargetVtbl) : IUnknown (IUnknownVtbl) { fn GetPackageTargetTypes (targetCount : * mut UINT32 , targetTypes : * mut * mut GUID ,) -> HRESULT , fn GetPackageTarget (guidTargetType : REFGUID , riid : REFIID , ppvTarget : * mut * mut c_void ,) -> HRESULT , fn Cancel () -> HRESULT , } }
};
}
