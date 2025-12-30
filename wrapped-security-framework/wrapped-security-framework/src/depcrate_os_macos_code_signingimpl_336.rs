// Generated macro for impl_336 (impl)
macro_rules! Depcrate_os_macos_code_signingimpl_336 {
() => {
// Module: crate::os::macos::code_signing
// Provides: {"impl_336"}
// Dependencies: {}
impl FromStr for SecRequirement { type Err = crate :: base :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let text = CFString :: new (s) ; let mut requirement = MaybeUninit :: uninit () ; unsafe { cvt (SecRequirementCreateWithString (text . as_concrete_TypeRef () , 0 , requirement . as_mut_ptr () ,)) ? ; Ok (Self :: wrap_under_create_rule (requirement . assume_init ())) } } }
};
}
