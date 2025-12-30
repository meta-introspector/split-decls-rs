// Generated macro for macro_38999 (macro)
macro_rules! Depcrate_um_taskschdmacro_38999 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38999"}
// Dependencies: {}
RIDL ! { # [uuid (0x10f62c64 , 0x7e16 , 0x4314 , 0xa0 , 0xc2 , 0x0c , 0x36 , 0x83 , 0xf9 , 0x9d , 0x40)] interface IEmailAction (IEmailActionVtbl) : IAction (IActionVtbl) { fn get_Server (pServer : * mut BSTR ,) -> HRESULT , fn put_Server (pServer : BSTR ,) -> HRESULT , fn get_Subject (pSubject : * mut BSTR ,) -> HRESULT , fn put_Subject (pSubject : BSTR ,) -> HRESULT , fn get_To (pTo : * mut BSTR ,) -> HRESULT , fn put_To (pTo : BSTR ,) -> HRESULT , fn get_Cc (pCc : * mut BSTR ,) -> HRESULT , fn put_Cc (pCc : BSTR ,) -> HRESULT , fn get_Bcc (pBcc : * mut BSTR ,) -> HRESULT , fn put_Bcc (pBcc : BSTR ,) -> HRESULT , fn get_ReplyTo (pReplyTo : * mut BSTR ,) -> HRESULT , fn put_ReplyTo (pReplyTo : BSTR ,) -> HRESULT , fn get_From (pFrom : * mut BSTR ,) -> HRESULT , fn put_From (pFrom : BSTR ,) -> HRESULT , fn get_HeaderFields (ppHeaderFields : * mut * mut ITaskNamedValueCollection ,) -> HRESULT , fn put_HeaderFields (ppHeaderFields : * const ITaskNamedValueCollection ,) -> HRESULT , fn get_Body (pBody : * mut BSTR ,) -> HRESULT , fn put_Body (pBody : BSTR ,) -> HRESULT , fn get_Attachments (pAttachements : * mut SAFEARRAY ,) -> HRESULT , fn put_Attachments (pAttachements : SAFEARRAY ,) -> HRESULT , } }
};
}
