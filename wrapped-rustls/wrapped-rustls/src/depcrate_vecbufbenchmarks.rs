// Generated macro for benchmarks (module)
macro_rules! Depcrate_vecbufbenchmarks {
() => {
// Module: crate::vecbuf
// Provides: {"benchmarks"}
// Dependencies: {}
# [cfg (bench)] mod benchmarks { use alloc :: vec ; use super :: ChunkVecBuffer ; # [bench] fn read_one_byte_from_large_message (b : & mut test :: Bencher) { b . iter (| | { let mut cvb = ChunkVecBuffer :: new (None) ; cvb . append (vec ! [0u8 ; 16_384]) ; assert_eq ! (1 , cvb . read (& mut [0u8]) . unwrap ()) ; }) ; } # [bench] fn read_all_individual_from_large_message (b : & mut test :: Bencher) { b . iter (| | { let mut cvb = ChunkVecBuffer :: new (None) ; cvb . append (vec ! [0u8 ; 16_384]) ; loop { if let Ok (0) = cvb . read (& mut [0u8]) { break ; } } }) ; } # [bench] fn read_half_bytes_from_large_message (b : & mut test :: Bencher) { b . iter (| | { let mut cvb = ChunkVecBuffer :: new (None) ; cvb . append (vec ! [0u8 ; 16_384]) ; assert_eq ! (8192 , cvb . read (& mut [0u8 ; 8192]) . unwrap ()) ; assert_eq ! (8192 , cvb . read (& mut [0u8 ; 8192]) . unwrap ()) ; }) ; } # [bench] fn read_entire_large_message (b : & mut test :: Bencher) { b . iter (| | { let mut cvb = ChunkVecBuffer :: new (None) ; cvb . append (vec ! [0u8 ; 16_384]) ; assert_eq ! (16_384 , cvb . read (& mut [0u8 ; 16_384]) . unwrap ()) ; }) ; } }
};
}
