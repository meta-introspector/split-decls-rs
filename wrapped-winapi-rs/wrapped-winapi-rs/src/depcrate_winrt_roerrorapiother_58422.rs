// Generated macro for other_58422 (other)
macro_rules! Depcrate_winrt_roerrorapiother_58422 {
() => {
// Module: crate::winrt::roerrorapi
// Provides: {"other_58422"}
// Dependencies: {}
extern "system" { pub fn RoInspectThreadErrorInfo (targetTebAddress : UINT_PTR , machine : USHORT , readMemoryCallback : PINSPECT_MEMORY_CALLBACK , context : PVOID , targetErrorInfoAddress : * mut UINT_PTR ,) -> HRESULT ; pub fn RoInspectCapturedStackBackTrace (targetErrorInfoAddress : UINT_PTR , machine : USHORT , readMemoryCallback : PINSPECT_MEMORY_CALLBACK , context : PVOID , frameCount : * mut UINT32 , targetBackTraceAddress : * mut UINT_PTR ,) -> HRESULT ; pub fn RoGetMatchingRestrictedErrorInfo (hrIn : HRESULT , ppRestrictedErrorInfo : * mut * mut IRestrictedErrorInfo ,) -> HRESULT ; pub fn RoReportFailedDelegate (punkDelegate : * const IUnknown , pRestrictedErrorInfo : * const IRestrictedErrorInfo ,) -> HRESULT ; pub fn IsErrorPropagationEnabled () -> BOOL ; }
};
}
