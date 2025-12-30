// Generated macro for macro_38958 (macro)
macro_rules! Depcrate_um_taskschdmacro_38958 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38958"}
// Dependencies: {}
RIDL ! { # [uuid (0x6a67614b , 0x6828 , 0x4fec , 0xaa , 0x54 , 0x6d , 0x52 , 0xe8 , 0xf1 , 0xf2 , 0xdb)] interface IRunningTaskCollection (IRunningTaskCollectionVtbl) : IDispatch (IDispatchVtbl) { fn get_Count (pCount : * mut LONG ,) -> HRESULT , fn get_Item (index : VARIANT , ppRunningTask : * mut * mut IRunningTask ,) -> HRESULT , fn get__NewEnum (ppEnum : * mut LPUNKNOWN ,) -> HRESULT , } }
};
}
