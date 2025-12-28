macro_rules! ShrinkDecoder {
    () => {
        # [derive (Debug)] pub struct ShrinkDecoder < R > { compressed_reader : R , stream_read : bool , uncompressed_size : u64 , stream : Vec < u8 > , read_pos : usize , }
    };
}

ShrinkDecoder!()