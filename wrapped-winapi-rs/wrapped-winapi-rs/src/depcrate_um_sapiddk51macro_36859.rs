// Generated macro for macro_36859 (macro)
macro_rules! Depcrate_um_sapiddk51macro_36859 {
() => {
// Module: crate::um::sapiddk51
// Provides: {"macro_36859"}
// Dependencies: {}
RIDL ! { # [uuid (0x9880499b , 0xcce9 , 0x11d2 , 0xb5 , 0x03 , 0x00 , 0xc0 , 0x4f , 0x79 , 0x73 , 0x96)] interface ISpTTSEngineSite (ISpTTSEngineSiteVtbl) : ISpEventSink (ISpEventSinkVtbl) { fn GetActions () -> DWORD , fn Write (pBuff : * const c_void , cb : ULONG , pcbWritten : * mut ULONG ,) -> HRESULT , fn GetRate (pRateAdjust : * mut c_long ,) -> HRESULT , fn GetVolume (pusVolume : * mut USHORT ,) -> HRESULT , fn GetSkipInfo (peType : * mut SPVSKIPTYPE , plNumItems : * mut c_long ,) -> HRESULT , fn CompleteSkip (ulNumSkipped : c_long ,) -> HRESULT , } }
};
}
