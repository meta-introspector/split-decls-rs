// Generated macro for macro_39632 (macro)
macro_rules! Depcrate_um_wbemclimacro_39632 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39632"}
// Dependencies: {}
RIDL ! { # [uuid (0xe7d35cfa , 0x348b , 0x485e , 0xb5 , 0x24 , 0x25 , 0x27 , 0x25 , 0xd6 , 0x97 , 0xca)] interface IWbemObjectSinkEx (IWbemObjectSinkExVtbl) : IWbemObjectSink (IWbemObjectSinkVtbl) { fn WriteMessage (uChannel : ULONG , strMessage : BSTR ,) -> HRESULT , fn WriteError (pObjError : * mut IWbemClassObject , puReturned : * mut c_uchar ,) -> HRESULT , fn PromptUser (strMessage : BSTR , uPromptType : c_uchar , puReturned : * mut c_uchar ,) -> HRESULT , fn WriteProgress (strActivity : BSTR , strCurrentOperation : BSTR , strStatusDescription : BSTR , uPercentComplete : ULONG , uSecondsRemaining : ULONG ,) -> HRESULT , fn WriteStreamParameter (strName : BSTR , vtValue : * mut VARIANT , ulType : ULONG , ulFlags : ULONG ,) -> HRESULT , } }
};
}
