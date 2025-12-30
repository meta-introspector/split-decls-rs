// Generated macro for macro_27315 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27315 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27315"}
// Dependencies: {}
RIDL ! { # [uuid (0xc2f8e001 , 0xf205 , 0x4bc9 , 0x99 , 0xbc , 0xc1 , 0x3b , 0x1e , 0x04 , 0x8c , 0xcb)] interface IPerChannelDbLevel (IPerChannelDbLevelVtbl) : IUnknown (IUnknownVtbl) { fn GetChannelCount (pcChannels : * mut UINT ,) -> HRESULT , fn GetLevelRange (nChannel : UINT , pfMinLevelDB : * mut c_float , pfMaxLevelDB : * mut c_float , pfStepping : * mut c_float ,) -> HRESULT , fn GetLevel (nChannel : UINT , pfLevelDB : * mut c_float ,) -> HRESULT , fn SetLevel (nChannel : UINT , fLevelDB : c_float , pguidEventContext : LPCGUID ,) -> HRESULT , fn SetLevelUniform (fLevelDB : c_float , pguidEventContext : LPCGUID ,) -> HRESULT , fn SetLevelAllChannels (aLevelsDB : * mut c_float , cChannels : ULONG , pguidEventContext : LPCGUID ,) -> HRESULT , } }
};
}
