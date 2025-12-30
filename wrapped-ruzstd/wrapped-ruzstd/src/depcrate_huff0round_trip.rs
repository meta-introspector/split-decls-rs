// Generated macro for round_trip (function)
macro_rules! Depcrate_huff0round_trip {
() => {
// Module: crate::huff0
// Provides: {"round_trip"}
// Dependencies: {}
# [doc = " Only needed for testing."] # [doc = ""] # [doc = " Encodes the data with a table built from that data"] # [doc = " Decodes the result again by first decoding the table and then the data"] # [doc = " Asserts that the decoded data equals the input"] # [cfg (any (test , feature = "fuzz_exports"))] pub fn round_trip (data : & [u8]) { use crate :: bit_io :: { BitReaderReversed , BitWriter } ; use alloc :: vec :: Vec ; if data . len () < 2 { return ; } if data . iter () . all (| x | * x == data [0]) { return ; } let mut writer = BitWriter :: new () ; let encoder_table = huff0_encoder :: HuffmanTable :: build_from_data (data) ; let mut encoder = huff0_encoder :: HuffmanEncoder :: new (& encoder_table , & mut writer) ; encoder . encode (data , true) ; let encoded = writer . dump () ; let mut decoder_table = HuffmanTable :: new () ; let table_bytes = decoder_table . build_decoder (& encoded) . unwrap () ; let mut decoder = HuffmanDecoder :: new (& decoder_table) ; let mut br = BitReaderReversed :: new (& encoded [table_bytes as usize ..]) ; let mut skipped_bits = 0 ; loop { let val = br . get_bits (1) ; skipped_bits += 1 ; if val == 1 || skipped_bits > 8 { break ; } } if skipped_bits > 8 { panic ! ("Corrupted end marker") ; } decoder . init_state (& mut br) ; let mut decoded = Vec :: new () ; while br . bits_remaining () > - (decoder_table . max_num_bits as isize) { decoded . push (decoder . decode_symbol ()) ; decoder . next_state (& mut br) ; } assert_eq ! (& decoded , data) ; }
};
}
