// Generated macro for macro_36757 (macro)
macro_rules! Depcrate_um_sapi53macro_36757 {
() => {
// Module: crate::um::sapi53
// Provides: {"macro_36757"}
// Dependencies: {}
RIDL ! { # [uuid (0x21b501a0 , 0x0ec7 , 0x46c9 , 0x92 , 0xc3 , 0xa2 , 0xbc , 0x78 , 0x4c , 0x54 , 0xb9)] interface ISpSerializeState (ISpSerializeStateVtbl) : IUnknown (IUnknownVtbl) { fn GetSerializedState (ppbData : * mut * mut BYTE , pulSize : * mut ULONG , dwReserved : DWORD ,) -> HRESULT , fn SetSerializedState (pbData : * mut BYTE , ulSize : ULONG , dwReserved : DWORD ,) -> HRESULT , } }
};
}
