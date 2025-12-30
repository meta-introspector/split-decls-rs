// Generated macro for macro_38959 (macro)
macro_rules! Depcrate_um_taskschdmacro_38959 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38959"}
// Dependencies: {}
RIDL ! { # [uuid (0xf5bc8fc5 , 0x536d , 0x4f77 , 0xb8 , 0x52 , 0xfb , 0xc1 , 0x35 , 0x6f , 0xde , 0xb6)] interface ITaskDefinition (ITaskDefinitionVtbl) : IDispatch (IDispatchVtbl) { fn get_RegistrationInfo (ppRegistrationInfo : * mut * mut IRegistrationInfo ,) -> HRESULT , fn put_RegistrationInfo (ppRegistrationInfo : * const IRegistrationInfo ,) -> HRESULT , fn get_Triggers (ppTriggers : * mut * mut ITriggerCollection ,) -> HRESULT , fn put_Triggers (ppTriggers : * const ITriggerCollection ,) -> HRESULT , fn get_Settings (ppSettings : * mut * mut ITaskSettings ,) -> HRESULT , fn put_Settings (ppSettings : * const ITaskSettings ,) -> HRESULT , fn get_Data (pData : * mut BSTR ,) -> HRESULT , fn put_Data (pData : BSTR ,) -> HRESULT , fn get_Principal (ppPrincipal : * mut * mut IPrincipal ,) -> HRESULT , fn put_Principal (ppPrincipal : * const IPrincipal ,) -> HRESULT , fn get_Actions (ppActions : * mut * mut IActionCollection ,) -> HRESULT , fn put_Actions (ppActions : * const IActionCollection ,) -> HRESULT , fn get_XmlText (pXml : * mut BSTR ,) -> HRESULT , fn put_XmlText (pXml : BSTR ,) -> HRESULT , } }
};
}
