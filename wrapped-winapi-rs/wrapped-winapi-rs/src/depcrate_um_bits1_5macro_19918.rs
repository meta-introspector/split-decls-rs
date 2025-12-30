// Generated macro for macro_19918 (macro)
macro_rules! Depcrate_um_bits1_5macro_19918 {
() => {
// Module: crate::um::bits1_5
// Provides: {"macro_19918"}
// Dependencies: {}
RIDL ! { # [uuid (0x54b50739 , 0x686f , 0x45eb , 0x9d , 0xff , 0xd6 , 0xa9 , 0xa0 , 0xfa , 0xa9 , 0xaf)] interface IBackgroundCopyJob2 (IBackgroundCopyJob2Vtbl) : IBackgroundCopyJob (IBackgroundCopyJobVtbl) { fn SetNotifyCmdLine (Program : LPCWSTR , Parameters : LPCWSTR ,) -> HRESULT , fn GetNotifyCmdLine (pProgram : * mut LPWSTR , pParameters : * mut LPWSTR ,) -> HRESULT , fn GetReplyProgress (pProgress : * mut BG_JOB_REPLY_PROGRESS ,) -> HRESULT , fn GetReplyData (ppBuffer : * mut * mut byte , pLength : * mut UINT64 ,) -> HRESULT , fn SetReplyFileName (ReplyFileName : LPCWSTR ,) -> HRESULT , fn GetReplyFileName (pReplyFileName : * mut LPWSTR ,) -> HRESULT , fn SetCredentials (credentials : * mut BG_AUTH_CREDENTIALS ,) -> HRESULT , fn RemoveCredentials (Target : BG_AUTH_TARGET , Scheme : BG_AUTH_SCHEME ,) -> HRESULT , } }
};
}
