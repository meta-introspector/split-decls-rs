// Generated macro for other_58420 (other)
macro_rules! Depcrate_winrt_roerrorapiother_58420 {
() => {
// Module: crate::winrt::roerrorapi
// Provides: {"other_58420"}
// Dependencies: {}
extern "system" { pub fn RoGetErrorReportingFlags (pflags : * mut UINT32 ,) -> HRESULT ; pub fn RoSetErrorReportingFlags (flags : UINT32 ,) -> HRESULT ; pub fn RoResolveRestrictedErrorInfoReference (reference : PCWSTR , ppRestrictedErrorInfo : * mut * mut IRestrictedErrorInfo ,) -> HRESULT ; pub fn SetRestrictedErrorInfo (pRestrictedErrorInfo : * const IRestrictedErrorInfo ,) -> HRESULT ; pub fn GetRestrictedErrorInfo (ppRestrictedErrorInfo : * mut * mut IRestrictedErrorInfo ,) -> HRESULT ; pub fn RoOriginateErrorW (error : HRESULT , cchMax : UINT , message : PCWSTR ,) -> BOOL ; pub fn RoOriginateError (error : HRESULT , message : HSTRING ,) -> BOOL ; pub fn RoTransformErrorW (oldError : HRESULT , newError : HRESULT , cchMax : UINT , message : PCWSTR ,) -> BOOL ; pub fn RoTransformError (oldError : HRESULT , newError : HRESULT , message : HSTRING ,) -> BOOL ; pub fn RoCaptureErrorContext (hr : HRESULT ,) -> HRESULT ; pub fn RoFailFastWithErrorContext (hrError : HRESULT ,) ; pub fn RoOriginateLanguageException (error : HRESULT , message : HSTRING , languageException : * const IUnknown ,) -> BOOL ; pub fn RoClearError () ; pub fn RoReportUnhandledError (pRestrictedErrorInfo : * const IRestrictedErrorInfo ,) -> HRESULT ; }
};
}
