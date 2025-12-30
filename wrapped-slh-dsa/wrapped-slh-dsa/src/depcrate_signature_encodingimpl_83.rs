// Generated macro for impl_83 (impl)
macro_rules! Depcrate_signature_encodingimpl_83 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_83"}
// Dependencies: {}
impl < P : ParameterSet > TryFrom < & [u8] > for Signature < P > { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { if bytes . len () != P :: SigLen :: USIZE { return Err (Error :: new ()) ; } let (rand_bytes , rest) = bytes . split_at (P :: N :: USIZE) ; # [allow (deprecated)] let randomizer = Array :: clone_from_slice (rand_bytes) ; let (fors_bytes , ht_bytes) = rest . split_at (ForsSignature :: < P > :: SIZE) ; let fors_sig = ForsSignature :: try_from (fors_bytes) . map_err (| () | Error :: new ()) ? ; let ht_sig = HypertreeSig :: try_from (ht_bytes) . map_err (| () | Error :: new ()) ? ; Ok (Signature { randomizer , fors_sig , ht_sig , }) } }
};
}
