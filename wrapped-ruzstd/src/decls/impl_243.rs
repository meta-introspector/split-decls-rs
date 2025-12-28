macro_rules! deps {
    () => {
        FrameHeader!();
        BitWriter!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl FrameHeader { # [doc = " Writes the serialized frame header into the provided buffer."] # [doc = ""] # [doc = " The returned header *does include* a frame header descriptor."] pub fn serialize (self , output : & mut Vec < u8 >) { vprintln ! ("Serializing frame with header: {self:?}") ; output . extend_from_slice (& MAGIC_NUM . to_le_bytes ()) ; output . push (self . descriptor ()) ; if ! self . single_segment { if let Some (window_size) = self . window_size { let log = window_size . next_power_of_two () . ilog2 () ; let exponent = if log > 10 { log - 10 } else { 1 } as u8 ; output . push (exponent << 3) ; } } if let Some (id) = self . dictionary_id { output . extend (minify_val (id)) ; } if let Some (frame_content_size) = self . frame_content_size { output . extend (minify_val_fcs (frame_content_size)) ; } } # [doc = " Generate a serialized frame header descriptor for the frame header."] # [doc = ""] # [doc = " https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#frame_header_descriptor"] fn descriptor (& self) -> u8 { let mut bw = BitWriter :: new () ; if let Some (id) = self . dictionary_id { let flag_value : u8 = match find_min_size (id) { 0 => 0 , 1 => 1 , 2 => 2 , 4 => 3 , _ => panic ! () , } ; bw . write_bits (flag_value , 2) ; } else { bw . write_bits (0u8 , 2) ; } if self . content_checksum { bw . write_bits (1u8 , 1) ; } else { bw . write_bits (0u8 , 1) ; } bw . write_bits (0u8 , 1) ; bw . write_bits (0u8 , 1) ; if self . single_segment { assert ! (self . frame_content_size . is_some () , "if the `single_segment` flag is set to true, then a frame content size must be provided") ; bw . write_bits (1u8 , 1) ; } else { assert ! (self . window_size . is_some () , "if the `single_segment` flag is set to false, then a window size must be provided") ; bw . write_bits (0u8 , 1) ; } if let Some (frame_content_size) = self . frame_content_size { let field_size = find_min_size (frame_content_size) ; let flag_value : u8 = match field_size { 1 => 0 , 2 => 1 , 4 => 2 , 3 => 8 , _ => panic ! () , } ; bw . write_bits (flag_value , 2) ; } else { bw . write_bits (0u8 , 2) ; } bw . dump () [0] } }
    };
}

impl_243!();