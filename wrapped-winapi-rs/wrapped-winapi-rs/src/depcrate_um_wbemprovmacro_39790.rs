// Generated macro for macro_39790 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39790 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39790"}
// Dependencies: {}
RIDL ! { # [uuid (0x3ae0080a , 0x7e3a , 0x4366 , 0xbf , 0x89 , 0x0f , 0xee , 0xdc , 0x93 , 0x16 , 0x59)] interface IWbemEventSink (IWbemEventSinkVtbl) : IWbemObjectSink (IWbemObjectSinkVtbl) { fn SetSinkSecurity (lSDLength : c_long , pSD : * mut BYTE ,) -> HRESULT , fn IsActive () -> HRESULT , fn GetRestrictedSink (lNumQueries : c_long , awszQueries : * const LPCWSTR , pCallback : * mut IUnknown , ppSink : * mut * mut IWbemEventSink ,) -> HRESULT , fn SetBatchingParameters (lFlags : LONG , dwMaxBufferSize : DWORD , dwMaxSendLatency : DWORD ,) -> HRESULT , } }
};
}
