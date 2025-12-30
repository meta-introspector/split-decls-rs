// Generated macro for macro_33954 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33954 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33954"}
// Dependencies: {}
RIDL ! { # [uuid (0x000001c0 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IContext (IContextVtbl) : IUnknown (IUnknownVtbl) { fn SetProperty (rpolicyId : REFGUID , flags : CPFLAGS , pUnk : * mut IUnknown ,) -> HRESULT , fn RemoveProperty (rPolicyId : REFGUID ,) -> HRESULT , fn GetProperty (policyId : REFGUID , pFlags : * mut CPFLAGS , ppUnk : * mut * mut IUnknown ,) -> HRESULT , fn EnumContextProps (ppEnumContextProps : * mut * mut IEnumContextProps ,) -> HRESULT , } }
};
}
