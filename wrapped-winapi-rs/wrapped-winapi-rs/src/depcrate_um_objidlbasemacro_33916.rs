// Generated macro for macro_33916 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33916 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33916"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000144 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IRpcOptions (IRpcOptionsVtbl) : IUnknown (IUnknownVtbl) { fn Set (pPrx : * mut IUnknown , dwProperty : RPCOPT_PROPERTIES , dwValue : ULONG_PTR ,) -> HRESULT , fn Query (pPrx : * mut IUnknown , dwProperty : RPCOPT_PROPERTIES , pdwValue : * mut ULONG_PTR ,) -> HRESULT , } }
};
}
