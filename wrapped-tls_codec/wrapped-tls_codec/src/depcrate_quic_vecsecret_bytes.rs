// Generated macro for secret_bytes (module)
macro_rules! Depcrate_quic_vecsecret_bytes {
() => {
// Module: crate::quic_vec
// Provides: {"secret_bytes"}
// Dependencies: {}
# [cfg (feature = "std")] mod secret_bytes { use super :: * ; use crate :: { Deserialize , Serialize } ; # [doc = " A wrapper struct around [`VLBytes`] that implements [`ZeroizeOnDrop`]. It"] # [doc = " behaves just like [`VLBytes`], except that it doesn't allow conversion into"] # [doc = " a [`Vec<u8>`]."] # [cfg_attr (feature = "serde" , derive (SerdeSerialize , SerdeDeserialize))] # [derive (Clone , PartialEq , Eq , Hash , Ord , PartialOrd , Zeroize , ZeroizeOnDrop)] pub struct SecretVLBytes (VLBytes) ; impl SecretVLBytes { # [doc = " Generate a new variable-length byte vector that implements"] # [doc = " [`ZeroizeOnDrop`]."] pub fn new (vec : Vec < u8 >) -> Self { Self (VLBytes { vec }) } fn vec (& self) -> & [u8] { & self . 0 . vec } fn vec_mut (& mut self) -> & mut Vec < u8 > { & mut self . 0 . vec } } impl_vl_bytes_generic ! (SecretVLBytes) ; impl Size for SecretVLBytes { fn tls_serialized_len (& self) -> usize { self . 0 . tls_serialized_len () } } impl DeserializeBytes for SecretVLBytes { fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > where Self : Sized , { let (bytes , remainder) = VLBytes :: tls_deserialize_bytes (bytes) ? ; Ok ((Self (bytes) , remainder)) } } impl Serialize for SecretVLBytes { fn tls_serialize < W : std :: io :: Write > (& self , writer : & mut W) -> Result < usize , Error > { self . 0 . tls_serialize (writer) } } impl Deserialize for SecretVLBytes { fn tls_deserialize < R : std :: io :: Read > (bytes : & mut R) -> Result < Self , Error > where Self : Sized , { Ok (Self (VLBytes :: tls_deserialize (bytes) ?)) } } }
};
}
