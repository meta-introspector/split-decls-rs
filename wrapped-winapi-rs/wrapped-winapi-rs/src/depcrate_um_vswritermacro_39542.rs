// Generated macro for macro_39542 (macro)
macro_rules! Depcrate_um_vswritermacro_39542 {
() => {
// Module: crate::um::vswriter
// Provides: {"macro_39542"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000000 , 0x0000 , 0x0000 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00)] interface IVssWMFiledesc (IVssWMFiledescVtbl) : IUnknown (IUnknownVtbl) { fn GetPath (pbstrPath : * mut BSTR ,) -> HRESULT , fn GetFilespec (pbstrFilespec : * mut BSTR ,) -> HRESULT , fn GetRecursive (pbRecursive : * mut bool ,) -> HRESULT , fn GetAlternateLocation (pbstrAlternateLocation : * mut BSTR ,) -> HRESULT , fn GetBackupTypeMask (pdwTypeMask : * mut DWORD ,) -> HRESULT , } }
};
}
