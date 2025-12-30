// Generated macro for macro_39441 (macro)
macro_rules! Depcrate_um_vssmacro_39441 {
() => {
// Module: crate::um::vss
// Provides: {"macro_39441"}
// Dependencies: {}
RIDL ! { # [uuid (0xae1c7110 , 0x2f60 , 0x11d3 , 0x8a , 0x39 , 0x00 , 0xc0 , 0x4f , 0x72 , 0xd8 , 0xe3)] interface IVssEnumObject (IVssEnumObjectVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut VSS_OBJECT_PROP , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppenum : * mut * mut IVssEnumObject ,) -> HRESULT , } }
};
}
