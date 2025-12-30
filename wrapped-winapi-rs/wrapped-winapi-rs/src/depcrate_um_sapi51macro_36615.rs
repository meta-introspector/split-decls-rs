// Generated macro for macro_36615 (macro)
macro_rules! Depcrate_um_sapi51macro_36615 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36615"}
// Dependencies: {}
RIDL ! { # [uuid (0x8be47b07 , 0x57f6 , 0x11d2 , 0x9e , 0xee , 0x00 , 0xc0 , 0x4f , 0x79 , 0x73 , 0x96)] interface ISpeechVoiceStatus (ISpeechVoiceStatusVtbl) : IDispatch (IDispatchVtbl) { fn get_CurrentStreamNumber (StreamNumber : * mut c_long ,) -> HRESULT , fn get_LastStreamNumberQueued (StreamNumber : * mut c_long ,) -> HRESULT , fn get_LastHResult (HResult : * mut c_long ,) -> HRESULT , fn get_RunningState (State : * mut SpeechRunState ,) -> HRESULT , fn get_InputWordPosition (Position : * mut c_long ,) -> HRESULT , fn get_InputWordLength (Length : * mut c_long ,) -> HRESULT , fn get_InputSentencePosition (Position : * mut c_long ,) -> HRESULT , fn get_InputSentenceLength (Length : * mut c_long ,) -> HRESULT , fn get_LastBookmark (Bookmark : * mut BSTR ,) -> HRESULT , fn get_LastBookmarkId (BookmarkId : * mut c_long ,) -> HRESULT , fn get_PhonemeId (PhoneId : * mut c_short ,) -> HRESULT , fn get_VisemeId (VisemeId : * mut c_short ,) -> HRESULT , } }
};
}
