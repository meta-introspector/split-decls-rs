// Generated macro for other_32028 (other)
macro_rules! Depcrate_um_mmdeviceapiother_32028 {
() => {
// Module: crate::um::mmdeviceapi
// Provides: {"other_32028"}
// Dependencies: {}
extern "system" { pub fn ActivateAudioInterfaceAsync (deviceInterfacePath : LPCWSTR , riid : REFIID , activationParams : * mut PROPVARIANT , completionHandler : * mut IActivateAudioInterfaceCompletionHandler , activationOperation : * mut * mut IActivateAudioInterfaceAsyncOperation ,) -> HRESULT ; }
};
}
