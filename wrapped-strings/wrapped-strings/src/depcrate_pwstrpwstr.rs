// Generated macro for PWSTR (struct)
macro_rules! Depcrate_pwstrPWSTR {
() => {
// Module: crate::pwstr
// Provides: {"PWSTR"}
// Dependencies: {}
# [doc = " A pointer to a null-terminated string of 16-bit Unicode characters."] # [repr (transparent)] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct PWSTR (pub * mut u16) ;
};
}
