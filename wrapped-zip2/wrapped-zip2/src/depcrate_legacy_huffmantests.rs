// Generated macro for tests (module)
macro_rules! Depcrate_legacy_huffmantests {
() => {
// Module: crate::legacy::huffman
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: io :: Cursor ; use bitstream_io :: { BitReader , LittleEndian } ; use super :: HuffmanDecoder ; # [test] fn test_huffman_decode_basic () { let lens = [3 , 3 , 3 , 3 , 3 , 3 , 4 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , 5 , 4 ,] ; let mut d = HuffmanDecoder :: default () ; d . init (& lens , lens . len ()) . unwrap () ; assert_eq ! (d . huffman_decode (8 , & mut BitReader :: endian (& mut Cursor :: new (& [! 0x0]) , LittleEndian)) . unwrap () , 0) ; assert_eq ! (d . huffman_decode (8 , & mut BitReader :: endian (& mut Cursor :: new (& [! 0b110]) , LittleEndian)) . unwrap () , 0b011) ; assert_eq ! (d . huffman_decode (8 , & mut BitReader :: endian (& mut Cursor :: new (& [! 0b1111]) , LittleEndian)) . unwrap () , 0b10001) ; assert_eq ! (d . huffman_decode (8 , & mut BitReader :: endian (& mut Cursor :: new (& [! 0b11111]) , LittleEndian)) . unwrap () , 0b10000) ; assert ! (d . huffman_decode (8 , & mut BitReader :: endian (& mut Cursor :: new (& [! 0x7f]) , LittleEndian)) . is_err ()) ; } }
};
}
