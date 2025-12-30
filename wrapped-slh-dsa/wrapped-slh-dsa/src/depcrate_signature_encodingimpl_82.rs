// Generated macro for impl_82 (impl)
macro_rules! Depcrate_signature_encodingimpl_82 {
() => {
// Module: crate::signature_encoding
// Provides: {"impl_82"}
// Dependencies: {}
impl < P : ParameterSet > Signature < P > { # [cfg (feature = "alloc")] # [doc = " Serialize the signature to a `Vec<u8>` of length `P::SigLen`."] pub fn to_vec (& self) -> alloc :: vec :: Vec < u8 > { let mut bytes = alloc :: vec :: Vec :: with_capacity (P :: SigLen :: USIZE) ; bytes . extend_from_slice (& self . randomizer) ; bytes . extend_from_slice (& self . fors_sig . to_vec ()) ; bytes . extend_from_slice (& self . ht_sig . to_vec ()) ; debug_assert ! (bytes . len () == P :: SigLen :: USIZE) ; bytes } # [doc = " Serialize the signature to a new stack-allocated array"] # [doc = " This clones the underlying fields"] pub fn to_bytes (& self) -> Array < u8 , P :: SigLen > { let mut bytes = Array :: < u8 , P :: SigLen > :: default () ; let r_size = P :: N :: USIZE ; let fors_size = ForsSignature :: < P > :: SIZE ; bytes [.. r_size] . copy_from_slice (& self . randomizer) ; self . fors_sig . write_to (& mut bytes [r_size .. r_size + fors_size]) ; self . ht_sig . write_to (& mut bytes [r_size + fors_size ..]) ; bytes } }
};
}
