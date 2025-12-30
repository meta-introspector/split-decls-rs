// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_services_redirectimpl_1080 {
() => {
// Module: crate::services::redirect
// Provides: {"impl_1080"}
// Dependencies: {}
impl < ResBody > fmt :: Debug for Redirect < ResBody > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Redirect") . field ("status_code" , & self . status_code) . field ("location" , & self . location) . finish () } }
};
}
