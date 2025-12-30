// Generated macro for macro_36861 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36861 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36861"}
// Dependencies: {}
RIDL ! { # [uuid (0xa74d7c8e , 0x4cc5 , 0x4f2f , 0xa6 , 0xeb , 0x80 , 0x4d , 0xee , 0x18 , 0x50 , 0x0e)] interface ISpTTSEngine (ISpTTSEngineVtbl) : IUnknown (IUnknownVtbl) { fn Speak (dwSpeakFlags : DWORD , rguidFormatId : REFGUID , pWaveFormatEx : * const WAVEFORMATEX , pTextFragList : * const SPVTEXTFRAG , pOutputSite : * mut ISpTTSEngineSite ,) -> HRESULT , fn GetOutputFormat (pTargetFmtId : * const GUID , pTargetWaveFormatEx : * const WAVEFORMATEX , pOutputFormatId : * mut GUID , ppCoMemOutputWaveFormatEx : * mut WAVEFORMATEX ,) -> HRESULT , } }
};
}
