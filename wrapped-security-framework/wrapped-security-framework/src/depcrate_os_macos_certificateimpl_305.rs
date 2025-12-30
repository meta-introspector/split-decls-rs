// Generated macro for impl_305 (impl)
macro_rules! Depcrate_os_macos_certificateimpl_305 {
() => {
// Module: crate::os::macos::certificate
// Provides: {"impl_305"}
// Dependencies: {}
impl Iterator for PropertySectionIter < '_ > { type Item = CertificateProperty ; # [inline] fn next (& mut self) -> Option < CertificateProperty > { self . 0 . next () . map (| t | CertificateProperty (t . clone ())) } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
