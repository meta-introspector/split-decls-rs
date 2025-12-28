macro_rules! deps {
    () => {
        HuffmanTable!();
        HuffmanEncoder!();
        BitWriter!();
    };
}

macro_rules! compress_literals {
    () => {
        deps!();
        fn compress_literals (literals : & [u8] , last_table : Option < & huff0_encoder :: HuffmanTable > , writer : & mut BitWriter < & mut Vec < u8 > > ,) -> Option < huff0_encoder :: HuffmanTable > { let reset_idx = writer . index () ; let new_encoder_table = huff0_encoder :: HuffmanTable :: build_from_data (literals) ; let (encoder_table , new_table) = if let Some (_table) = last_table { if let Some (diff) = _table . can_encode (& new_encoder_table) { if diff > 5 { (& new_encoder_table , true) } else { (_table , false) } } else { (& new_encoder_table , true) } } else { (& new_encoder_table , true) } ; if new_table { writer . write_bits (2u8 , 2) ; } else { writer . write_bits (3u8 , 2) ; } let (size_format , size_bits) = match literals . len () { 0 .. 6 => (0b00u8 , 10) , 6 .. 1024 => (0b01 , 10) , 1024 .. 16384 => (0b10 , 14) , 16384 .. 262144 => (0b11 , 18) , _ => unimplemented ! ("too many literals") , } ; writer . write_bits (size_format , 2) ; writer . write_bits (literals . len () as u32 , size_bits) ; let size_index = writer . index () ; writer . write_bits (0u32 , size_bits) ; let index_before = writer . index () ; let mut encoder = huff0_encoder :: HuffmanEncoder :: new (encoder_table , writer) ; if size_format == 0 { encoder . encode (literals , new_table) } else { encoder . encode4x (literals , new_table) } ; let encoded_len = (writer . index () - index_before) / 8 ; writer . change_bits (size_index , encoded_len as u64 , size_bits) ; let total_len = (writer . index () - reset_idx) / 8 ; if total_len >= literals . len () { writer . reset_to (reset_idx) ; raw_literals (literals , writer) ; None } else if new_table { Some (new_encoder_table) } else { None } }
    };
}

compress_literals!()