// Generated macro for SecAuthenticationType (enum)
macro_rules! Depcrate_keychainSecAuthenticationType {
() => {
// Module: crate::keychain
// Provides: {"SecAuthenticationType"}
// Dependencies: {}
# [repr (u32)] # [derive (Copy , Clone , Eq , PartialEq , Debug)] # [allow (clippy :: upper_case_acronyms)] pub enum SecAuthenticationType { NTLM = char_lit_swapped ! (b"ntlm") , MSN = char_lit_swapped ! (b"msna") , DPA = char_lit_swapped ! (b"dpaa") , RPA = char_lit_swapped ! (b"rpaa") , HTTPBasic = char_lit_swapped ! (b"http") , HTTPDigest = char_lit_swapped ! (b"httd") , HTMLForm = char_lit_swapped ! (b"form") , Default = char_lit_swapped ! (b"dflt") , Any = 0 , }
};
}
