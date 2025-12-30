// Generated macro for macro_34094 (macro)
macro_rules! Depcrate_um_opmapimacro_34094 {
() => {
// Module: crate::um::opmapi
// Provides: {"macro_34094"}
// Dependencies: {}
RIDL ! { # [uuid (0x0a15159d , 0x41c7 , 0x4456 , 0x93 , 0xe1 , 0x28 , 0x4c , 0xd6 , 0x1d , 0x4e , 0x8d)] interface IOPMVideoOutput (IOPMVideoOutputVtbl) : IUnknown (IUnknownVtbl) { fn StartInitialization (prnRandomNumber : * mut OPM_RANDOM_NUMBER , ppbCertificate : * mut * mut BYTE , pulCertificateLength : * mut ULONG ,) -> HRESULT , fn FinishInitialization (pParameters : * const OPM_ENCRYPTED_INITIALIZATION_PARAMETERS ,) -> HRESULT , fn GetInformation (pParameters : * const OPM_GET_INFO_PARAMETERS , pRequestedInformation : * mut OPM_REQUESTED_INFORMATION ,) -> HRESULT , fn COPPCompatibleGetInformation (pParameters : * const OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS , pRequestedInformation : * mut OPM_REQUESTED_INFORMATION ,) -> HRESULT , fn Configure (pParameters : * const OPM_CONFIGURE_PARAMETERS , ulAdditionalParametersSize : ULONG , pbAdditionalParameters : * const BYTE ,) -> HRESULT , } }
};
}
