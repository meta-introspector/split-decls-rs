// Generated macro for macro_19960 (macro)
macro_rules! Depcrate_um_bits3_0macro_19960 {
() => {
// Module: crate::um::bits3_0
// Provides: {"macro_19960"}
// Dependencies: {}
RIDL ! { # [uuid (0x659cdea4 , 0x489e , 0x11d9 , 0xa9 , 0xcd , 0x00 , 0x0d , 0x56 , 0x96 , 0x52 , 0x51)] interface IEnumBitsPeerCacheRecords (IEnumBitsPeerCacheRecordsVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut * mut IBitsPeerCacheRecord , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumBitsPeerCacheRecords ,) -> HRESULT , fn GetCount (puCount : * mut ULONG ,) -> HRESULT , } }
};
}
