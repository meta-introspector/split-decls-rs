// Generated macro for other_58405 (other)
macro_rules! Depcrate_winrt_roapiother_58405 {
() => {
// Module: crate::winrt::roapi
// Provides: {"other_58405"}
// Dependencies: {}
extern "system" { pub fn RoInitialize (initType : RO_INIT_TYPE ,) -> HRESULT ; pub fn RoUninitialize () ; pub fn RoActivateInstance (activatableClassId : HSTRING , instance : * mut * mut IInspectable ,) -> HRESULT ; pub fn RoRegisterActivationFactories (activatableClassIds : * const HSTRING , activationFactoryCallbacks : * const PFNGETACTIVATIONFACTORY , count : UINT32 , cookie : * mut RO_REGISTRATION_COOKIE ,) -> HRESULT ; pub fn RoRevokeActivationFactories (cookie : RO_REGISTRATION_COOKIE ,) ; pub fn RoGetActivationFactory (activatableClassId : HSTRING , iid : REFIID , factory : * mut * mut VOID ,) -> HRESULT ; }
};
}
