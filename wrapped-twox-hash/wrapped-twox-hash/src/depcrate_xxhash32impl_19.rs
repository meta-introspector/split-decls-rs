// Generated macro for impl_19 (impl)
macro_rules! Depcrate_xxhash32impl_19 {
() => {
// Module: crate::xxhash32
// Provides: {"impl_19"}
// Dependencies: {}
impl Buffer { const fn new () -> Self { Self { offset : 0 , data : BufferData :: new () , } } # [inline] fn extend < 'd > (& mut self , data : & 'd [u8]) -> (Option < & Lanes > , & 'd [u8]) { if self . offset == 0 { return (None , data) ; } ; let bytes = self . data . bytes_mut () ; debug_assert ! (self . offset <= bytes . len ()) ; let empty = & mut bytes [self . offset ..] ; let n_to_copy = usize :: min (empty . len () , data . len ()) ; let dst = & mut empty [.. n_to_copy] ; let (src , rest) = data . split_at (n_to_copy) ; dst . copy_from_slice (src) ; self . offset += n_to_copy ; debug_assert ! (self . offset <= bytes . len ()) ; if self . offset == bytes . len () { self . offset = 0 ; (Some (& self . data . 0) , rest) } else { (None , rest) } } # [inline] fn set (& mut self , data : & [u8]) { if data . is_empty () { return ; } debug_assert_eq ! (self . offset , 0) ; let n_to_copy = data . len () ; let bytes = self . data . bytes_mut () ; debug_assert ! (n_to_copy < bytes . len ()) ; bytes [.. n_to_copy] . copy_from_slice (data) ; self . offset = data . len () ; } # [inline] fn remaining (& self) -> & [u8] { & self . data . bytes () [.. self . offset] } }
};
}
