// Generated macro for is_chrome_only (function)
macro_rules! Depcrate_utilis_chrome_only {
() => {
// Module: crate::util
// Provides: {"is_chrome_only"}
// Dependencies: {}
# [doc = " ChromeOnly is for things that are only exposed to privileged code in Firefox."] pub fn is_chrome_only (ext_attrs : & Option < ExtendedAttributeList >) -> bool { has_named_attribute (ext_attrs . as_ref () , "ChromeOnly") }
};
}
