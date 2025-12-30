// Generated macro for macro_36791 (macro)
macro_rules! Depcrate_um_sapi53macro_36791 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36791"}
// Dependencies: {}
RIDL ! { # [uuid (0xaaec54af , 0x8f85 , 0x4924 , 0x94 , 0x4d , 0xb7 , 0x9d , 0x39 , 0xd7 , 0x2e , 0x19)] interface ISpeechXMLRecoResult (ISpeechXMLRecoResultVtbl) : ISpeechRecoResult (ISpeechRecoResultVtbl) { fn GetXMLResult (Options : SPXMLRESULTOPTIONS , pResult : * mut BSTR ,) -> HRESULT , fn GetXMLErrorInfo (LineNumber : * mut c_long , ScriptLine : * mut BSTR , Source : * mut BSTR , Description : * mut BSTR , ResultCode : * mut c_long , IsError : * mut VARIANT_BOOL ,) -> HRESULT , } }
};
}
