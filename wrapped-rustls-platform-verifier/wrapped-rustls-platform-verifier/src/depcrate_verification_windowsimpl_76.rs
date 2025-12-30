// Generated macro for impl_76 (impl)
macro_rules! Depcrate_verification_windowsimpl_76 {
() => {
// Module: crate::verification::windows
// Provides: {"impl_76"}
// Dependencies: {}
impl Certificate { # [doc = " Sets the specified property of this certificate context."] # [doc = ""] # [doc = " ### Safety"] # [doc = " `prop_data` must be a valid pointer for the property type."] unsafe fn set_property (& mut self , prop_id : u32 , prop_data : * const c_void ,) -> Result < () , TlsError > { call_with_last_error (| | { (CertSetCertificateContextProperty (self . inner . as_ptr () , prop_id , CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG , prop_data ,) == TRUE) . then_some (()) }) } }
};
}
