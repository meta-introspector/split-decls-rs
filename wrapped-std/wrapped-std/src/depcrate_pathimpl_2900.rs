// Generated macro for impl_2900 (impl)
macro_rules! Depcrate_pathimpl_2900 {
() => {
// Module: crate::path
// Provides: {"impl_2900"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Hash for Path { fn hash < H : Hasher > (& self , h : & mut H) { let bytes = self . as_u8_slice () ; let (prefix_len , verbatim) = match parse_prefix (& self . inner) { Some (prefix) => { prefix . hash (h) ; (prefix . len () , prefix . is_verbatim ()) } None => (0 , false) , } ; let bytes = & bytes [prefix_len ..] ; let mut component_start = 0 ; let mut chunk_bits : usize = 0 ; for i in 0 .. bytes . len () { let is_sep = if verbatim { is_verbatim_sep (bytes [i]) } else { is_sep_byte (bytes [i]) } ; if is_sep { if i > component_start { let to_hash = & bytes [component_start .. i] ; chunk_bits = chunk_bits . wrapping_add (to_hash . len ()) ; chunk_bits = chunk_bits . rotate_right (2) ; h . write (to_hash) ; } component_start = i + 1 ; let tail = & bytes [component_start ..] ; if ! verbatim { component_start += match tail { [b'.'] => 1 , [b'.' , sep , ..] if is_sep_byte (* sep) => 1 , _ => 0 , } ; } } } if component_start < bytes . len () { let to_hash = & bytes [component_start ..] ; chunk_bits = chunk_bits . wrapping_add (to_hash . len ()) ; chunk_bits = chunk_bits . rotate_right (2) ; h . write (to_hash) ; } h . write_usize (chunk_bits) ; } }
};
}
