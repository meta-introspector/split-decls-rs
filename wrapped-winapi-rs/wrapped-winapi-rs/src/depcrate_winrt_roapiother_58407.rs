// Generated macro for other_58407 (other)
macro_rules! Depcrate_winrt_roapiother_58407 {
() => {
// Module: crate::winrt::roapi
// Provides: {"other_58407"}
// Dependencies: {}
extern "system" { pub fn RoRegisterForApartmentShutdown (callbackObject : * const IApartmentShutdown , apartmentIdentifier : * mut UINT64 , regCookie : * mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE ,) -> HRESULT ; pub fn RoUnregisterForApartmentShutdown (regCookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE ,) -> HRESULT ; pub fn RoGetApartmentIdentifier (apartmentIdentifier : * mut UINT64 ,) -> HRESULT ; }
};
}
