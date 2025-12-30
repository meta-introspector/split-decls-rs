// Generated macro for impl_343 (impl)
macro_rules! Depcrate_os_macos_code_signingimpl_343 {
() => {
// Module: crate::os::macos::code_signing
// Provides: {"impl_343"}
// Dependencies: {}
impl SecStaticCode { # [doc = " Creates a static code object representing the code at a specified file"] # [doc = " system path."] pub fn from_path (path : & CFURL , flags : Flags) -> Result < Self > { let mut code = MaybeUninit :: uninit () ; unsafe { cvt (SecStaticCodeCreateWithPath (path . as_concrete_TypeRef () , flags . bits () , code . as_mut_ptr () ,)) ? ; Ok (Self :: wrap_under_create_rule (code . assume_init ())) } } # [doc = " Retrieves the location on disk of signed code, given a code or static"] # [doc = " code object."] pub fn path (& self , flags : Flags) -> Result < CFURL > { let mut url = MaybeUninit :: uninit () ; unsafe { cvt (SecCodeCopyPath (self . as_concrete_TypeRef () , flags . bits () , url . as_mut_ptr () ,)) ? ; Ok (CFURL :: wrap_under_create_rule (url . assume_init ())) } } # [doc = " Performs dynamic validation of signed code."] pub fn check_validity (& self , flags : Flags , requirement : & SecRequirement) -> Result < () > { unsafe { cvt (SecStaticCodeCheckValidity (self . as_concrete_TypeRef () , flags . bits () , requirement . as_concrete_TypeRef () ,)) } } }
};
}
