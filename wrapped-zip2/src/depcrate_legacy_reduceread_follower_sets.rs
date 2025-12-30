// Generated macro for read_follower_sets (function)
macro_rules! Depcrate_legacy_reduceread_follower_sets {
() => {
// Module: crate::legacy::reduce
// Provides: {"read_follower_sets"}
// Dependencies: {}
fn read_follower_sets < T : std :: io :: Read , E : Endianness > (is : & mut BitReader < T , E > ,) -> io :: Result < FollowerSetArray > { let mut fsets = [FollowerSet :: default () ; u8 :: MAX as usize + 1] ; for i in (0 ..= u8 :: MAX as usize) . rev () { let n = is . read :: < 6 , u8 > () ? ; if n > 32 { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "invalid follower set" ,)) ; } fsets [i] . size = n ; fsets [i] . idx_bitlen = follower_idx_bitlen (n) ; for j in 0 .. fsets [i] . size as usize { fsets [i] . followers [j] = is . read :: < 8 , u8 > () ? ; } } Ok (fsets) }
};
}
