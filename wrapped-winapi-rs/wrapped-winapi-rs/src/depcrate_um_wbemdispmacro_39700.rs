// Generated macro for macro_39700 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39700 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39700"}
// Dependencies: {}
RIDL ! { # [uuid (0xb54d66e6 , 0x2287 , 0x11d2 , 0x8b , 0x33 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemSecurity (ISWbemSecurityVtbl) : IDispatch (IDispatchVtbl) { fn get_ImpersonationLevel (iImpersonationLevel : * mut WbemImpersonationLevelEnum ,) -> HRESULT , fn put_ImpersonationLevel (iImpersonationLevel : WbemImpersonationLevelEnum ,) -> HRESULT , fn get_AuthenticationLevel (iAuthenticationLevel : * mut WbemAuthenticationLevelEnum ,) -> HRESULT , fn put_AuthenticationLevel (iAuthenticationLevel : WbemAuthenticationLevelEnum ,) -> HRESULT , fn get_Privileges (objWbemPrivilegeSet : * mut * mut ISWbemPrivilegeSet ,) -> HRESULT , } }
};
}
