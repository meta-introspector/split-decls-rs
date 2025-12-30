// Generated macro for encode_varule_to_box (function)
macro_rules! Depcrate_ule_encodeencode_varule_to_box {
() => {
// Module: crate::ule::encode
// Provides: {"encode_varule_to_box"}
// Dependencies: {}
# [doc = " Given an [`EncodeAsVarULE`] type `S`, encode it into a `Box<T>`"] # [doc = ""] # [doc = " This is primarily useful for generating `Deserialize` impls for VarULE types"] # [cfg (feature = "alloc")] pub fn encode_varule_to_box < S : EncodeAsVarULE < T > + ? Sized , T : VarULE + ? Sized > (x : & S) -> Box < T > { let mut vec : Vec < u8 > = vec ! [0 ; x . encode_var_ule_len ()] ; x . encode_var_ule_write (& mut vec) ; let boxed = mem :: ManuallyDrop :: new (vec . into_boxed_slice ()) ; unsafe { let ptr : * mut T = T :: from_bytes_unchecked (& boxed) as * const T as * mut T ; Box :: from_raw (ptr) } }
};
}
