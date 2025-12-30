// Generated macro for macro_38394 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38394 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38394"}
// Dependencies: {}
RIDL ! { # [uuid (0x43826d1e , 0xe718 , 0x42ee , 0xbc , 0x55 , 0xa1 , 0xe2 , 0x61 , 0xc3 , 0x7b , 0xfe)] interface IShellItem (IShellItemVtbl) : IUnknown (IUnknownVtbl) { fn BindToHandler (pbc : * mut IBindCtx , bhid : REFGUID , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , fn GetParent (ppsi : * mut * mut IShellItem ,) -> HRESULT , fn GetDisplayName (sigdnName : SIGDN , ppszName : * mut LPWSTR ,) -> HRESULT , fn GetAttributes (sfgaoMask : SFGAOF , psfgaoAttribs : * mut SFGAOF ,) -> HRESULT , fn Compare (psi : * mut IShellItem , hint : SICHINTF , piOrder : * mut c_int ,) -> HRESULT , } }
};
}
