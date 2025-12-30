// Generated macro for macro_19962 (macro)
macro_rules! Depcrate_um_bits3_0macro_19962 {
() => {
// Module: crate::um::bits3_0
// Provides: {"macro_19962"}
// Dependencies: {}
RIDL ! { # [uuid (0x659cdea5 , 0x489e , 0x11d9 , 0xa9 , 0xcd , 0x00 , 0x0d , 0x56 , 0x96 , 0x52 , 0x51)] interface IEnumBitsPeers (IEnumBitsPeersVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IBitsPeer , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumBitsPeers ,) -> HRESULT , fn GetCount (puCount : * mut ULONG ,) -> HRESULT , } }
};
}
