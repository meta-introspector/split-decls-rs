// Generated macro for impl_104 (impl)
macro_rules! Depcrate_certificateimpl_104 {
() => {
// Module: crate::certificate
// Provides: {"impl_104"}
// Dependencies: {}
impl fmt :: Debug for SecCertificate { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("SecCertificate") . field ("subject" , & self . subject_summary ()) . finish () } }
};
}
