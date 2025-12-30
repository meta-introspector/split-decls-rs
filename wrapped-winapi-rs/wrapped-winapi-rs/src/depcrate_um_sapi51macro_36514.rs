// Generated macro for macro_36514 (macro)
macro_rules! Depcrate_um_sapi51macro_36514 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36514"}
// Dependencies: {}
RIDL ! { # [uuid (0xc2b5f241 , 0xdaa0 , 0x4507 , 0x9e , 0x16 , 0x5a , 0x1e , 0xaa , 0x2b , 0x7a , 0x5c)] interface ISpRecognizer (ISpRecognizerVtbl) : ISpProperties (ISpPropertiesVtbl) { fn SetRecognizer (pRecognizer : * mut ISpObjectToken ,) -> HRESULT , fn GetRecognizer (ppRecognizer : * mut * mut ISpObjectToken ,) -> HRESULT , fn SetInput (pUnkInput : * mut IUnknown , fAllowFormatChanges : BOOL ,) -> HRESULT , fn GetInputObjectToken (ppToken : * mut * mut ISpObjectToken ,) -> HRESULT , fn GetInputStream (ppStream : * mut * mut ISpStreamFormat ,) -> HRESULT , fn CreateRecoContext (ppNewCtxt : * mut * mut ISpRecoContext ,) -> HRESULT , fn GetRecoProfile (ppToken : * mut * mut ISpObjectToken ,) -> HRESULT , fn SetRecoProfile (pToken : * mut ISpObjectToken ,) -> HRESULT , fn IsSharedInstance () -> HRESULT , fn GetRecoState (pState : * mut SPRECOSTATE ,) -> HRESULT , fn SetRecoState (NewState : SPRECOSTATE ,) -> HRESULT , fn GetStatus (pStatus : * mut SPRECOGNIZERSTATUS ,) -> HRESULT , fn GetFormat (WaveFormatType : SPSTREAMFORMATTYPE , pFormatId : * mut GUID , ppCoMemWFEX : * mut WAVEFORMATEX ,) -> HRESULT , fn IsUISupported (pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG , pfSupported : * mut BOOL ,) -> HRESULT , fn DisplayUI (hwndParent : HWND , pszTitle : LPCWSTR , pszTypeOfUI : LPCWSTR , pvExtraData : * mut c_void , cbExtraData : ULONG ,) -> HRESULT , fn EmulateRecognition (pPhrase : * mut ISpPhrase ,) -> HRESULT , } }
};
}
