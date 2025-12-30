// Generated macro for impl_718 (impl)
macro_rules! Depcrate_trust_settingsimpl_718 {
() => {
// Module: crate::trust_settings
// Provides: {"impl_718"}
// Dependencies: {}
impl Iterator for TrustSettingsIter { type Item = SecCertificate ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . index >= self . array . len () { None } else { let cert = self . array . get (self . index) . unwrap () ; self . index += 1 ; Some (cert . clone ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let left = (self . array . len () as usize) . saturating_sub (self . index as usize) ; (left , Some (left)) } }
};
}
