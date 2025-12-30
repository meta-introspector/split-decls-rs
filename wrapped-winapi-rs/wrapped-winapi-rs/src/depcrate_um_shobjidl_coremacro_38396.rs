// Generated macro for macro_38396 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38396 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38396"}
// Dependencies: {}
RIDL ! { # [uuid (0xb63ea76d , 0x1f85 , 0x456f , 0xa1 , 0x9c , 0x48 , 0x15 , 0x9e , 0xfa , 0x85 , 0x8b)] interface IShellItemArray (IShellItemArrayVtbl) : IUnknown (IUnknownVtbl) { fn BindToHandler (pbc : * mut IBindCtx , bhid : REFGUID , riid : REFIID , ppvOut : * mut * mut c_void ,) -> HRESULT , fn GetPropertyStore (flags : GETPROPERTYSTOREFLAGS , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , fn GetPropertyDescriptionList (keyType : REFPROPERTYKEY , riid : REFIID , ppv : * mut * mut c_void ,) -> HRESULT , fn GetAttributes (AttribFlags : SIATTRIBFLAGS , sfgaoMask : SFGAOF , psfgaoAttribs : * mut SFGAOF ,) -> HRESULT , fn GetCount (pdwNumItems : * mut DWORD ,) -> HRESULT , fn GetItemAt (dwIndex : DWORD , ppsi : * mut * mut IShellItem ,) -> HRESULT , } }
};
}
