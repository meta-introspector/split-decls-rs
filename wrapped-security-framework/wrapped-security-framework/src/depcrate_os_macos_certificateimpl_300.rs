// Generated macro for impl_300 (impl)
macro_rules! Depcrate_os_macos_certificateimpl_300 {
() => {
// Module: crate::os::macos::certificate
// Provides: {"impl_300"}
// Dependencies: {}
impl CertificateProperty { # [doc = " Returns the label of this property."] # [must_use] pub fn label (& self) -> CFString { unsafe { CFString :: wrap_under_get_rule ((* self . 0 . get (kSecPropertyKeyLabel . to_void ())) . cast ()) } } # [doc = " Returns an enum of the underlying data for this property."] # [must_use] pub fn get (& self) -> PropertyType { unsafe { let type_ = CFString :: wrap_under_get_rule (* self . 0 . get (kSecPropertyKeyType . to_void ()) as * mut _) ; let value = self . 0 . get (kSecPropertyKeyValue . to_void ()) ; if type_ == CFString :: wrap_under_get_rule (kSecPropertyTypeSection) { PropertyType :: Section (PropertySection (CFArray :: wrap_under_get_rule ((* value) . cast ()))) } else if type_ == CFString :: wrap_under_get_rule (kSecPropertyTypeString) { PropertyType :: String (CFString :: wrap_under_get_rule ((* value) . cast ())) } else { PropertyType :: __Unknown } } } }
};
}
