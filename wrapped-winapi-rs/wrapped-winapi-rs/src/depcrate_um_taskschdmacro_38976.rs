// Generated macro for macro_38976 (macro)
macro_rules! Depcrate_um_taskschdmacro_38976 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38976"}
// Dependencies: {}
RIDL ! { # [uuid (0x86627eb4 , 0x42a7 , 0x41e4 , 0xa4 , 0xd9 , 0xac , 0x33 , 0xa7 , 0x2f , 0x2d , 0x52)] interface IRegisteredTaskCollection (IRegisteredTaskCollectionVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (pCount : * mut LONG ,) -> HRESULT , fn get_Item (index : VARIANT , ppRegisteredTask : * mut * mut IRegisteredTask ,) -> HRESULT , fn get__NewEnum (ppEnum : * mut LPUNKNOWN ,) -> HRESULT , } }
};
}
