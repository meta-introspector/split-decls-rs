// Generated macro for VLBytes (struct)
macro_rules! Depcrate_quic_vecVLBytes {
() => {
// Module: crate::quic_vec
// Provides: {"VLBytes"}
// Dependencies: {}
# [doc = " Variable-length encoded byte vectors."] # [doc = " Use this struct if bytes are encoded."] # [doc = " This is faster than the generic version."] # [cfg_attr (feature = "serde" , derive (SerdeSerialize , SerdeDeserialize))] # [cfg_attr (feature = "std" , derive (Zeroize))] # [derive (Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct VLBytes { # [cfg_attr (feature = "serde" , serde (serialize_with = "serde_bytes::serialize"))] # [cfg_attr (feature = "serde" , serde (deserialize_with = "serde_impl::de_vec_bytes_compat"))] vec : Vec < u8 > , }
};
}
