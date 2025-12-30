// Generated macro for macro_39546 (macro)
macro_rules! Depcrate_um_vswritermacro_39546 {
() => {
// Module: crate::um::vswriter
// Provides: {"macro_39546"}
// Dependencies: {}
RIDL ! { # [uuid (0x156c8b5e , 0xf131 , 0x4bd7 , 0x9c , 0x97 , 0xd1 , 0x92 , 0x3b , 0xe7 , 0xe1 , 0xfa)] interface IVssComponentEx (IVssComponentExVtbl) : IVssComponent (IVssComponentVtbl) { fn SetPrepareForBackupFailureMsg (wszFailureMsg : LPCWSTR ,) -> HRESULT , fn SetPostSnapshotFailureMsg (wszFailureMsg : LPCWSTR ,) -> HRESULT , fn GetPrepareForBackupFailureMsg (pbstrFailureMsg : * mut BSTR ,) -> HRESULT , fn GetPostSnapshotFailureMsg (pbstrFailureMsg : * mut BSTR ,) -> HRESULT , fn GetAuthoritativeRestore (pbAuth : * mut bool ,) -> HRESULT , fn GetRollForward (pRollType : * mut VSS_ROLLFORWARD_TYPE , pbstrPoint : * mut BSTR ,) -> HRESULT , fn GetRestoreName (pbstrName : * mut BSTR ,) -> HRESULT , } }
};
}
