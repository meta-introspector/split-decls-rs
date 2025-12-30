// Generated macro for impl_332 (impl)
macro_rules! Depcrate_os_macos_code_signingimpl_332 {
() => {
// Module: crate::os::macos::code_signing
// Provides: {"impl_332"}
// Dependencies: {}
impl GuestAttributes { # [doc = " Creates a new, empty `GuestAttributes`. You must add values to it in"] # [doc = " order for it to be of any use."] # [must_use] pub fn new () -> Self { Self { inner : CFMutableDictionary :: new () , } } # [doc = " The guest's audit token."] pub fn set_audit_token (& mut self , token : CFDataRef) { let key = unsafe { CFString :: wrap_under_get_rule (kSecGuestAttributeAudit) } ; self . inner . add (& key . as_CFTypeRef () , & token . to_void ()) ; } # [doc = " The guest's pid."] pub fn set_pid (& mut self , pid : pid_t) { let key = unsafe { CFString :: wrap_under_get_rule (kSecGuestAttributePid) } ; let pid = CFNumber :: from (pid) ; self . inner . add (& key . as_CFTypeRef () , & pid . as_CFTypeRef ()) ; } # [doc = " Support for arbirtary guest attributes."] pub fn set_other < V : ToVoid < V > > (& mut self , key : CFStringRef , value : V) { self . inner . add (& key . as_void_ptr () , & value . to_void ()) ; } }
};
}
