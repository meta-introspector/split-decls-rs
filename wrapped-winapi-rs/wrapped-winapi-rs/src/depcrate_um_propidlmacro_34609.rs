// Generated macro for macro_34609 (macro)
macro_rules! Depcrate_um_propidlmacro_34609 {
() => {
// Module: crate::um::propidl
// Provides: {"macro_34609"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000013B , 0x0000 , 0x0000 , 0xC0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumSTATPROPSETSTG (IEnumSTATPROPSETSTGVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , rgelt : * mut STATPROPSETSTG , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Revert () -> HRESULT , fn Clone (ppenum : * mut * mut IEnumSTATPROPSETSTG ,) -> HRESULT , } }
};
}
