// Generated macro for macro_38979 (macro)
macro_rules! Depcrate_um_taskschdmacro_38979 {
() => {
// Module: crate::um::taskschd
// Provides: {"macro_38979"}
// Dependencies: {}
RIDL ! { # [uuid (0xeaec7a8f , 0x27a0 , 0x4ddc , 0x86 , 0x75 , 0x14 , 0x72 , 0x6a , 0x01 , 0xa3 , 0x8a)] interface ITaskHandlerStatus (ITaskHandlerStatusVtbl) : IUnknown (IUnknownVtbl) { fn UpdateStatus (percentComplete : c_short , statusMessage : BSTR ,) -> HRESULT , fn TaskCompleted (taskErrCode : HRESULT ,) -> HRESULT , } }
};
}
