macro_rules! ReduceDecoder {
    () => {
        # [derive (Debug)] pub struct ReduceDecoder < R > { compressed_reader : R , uncompressed_size : u64 , stream_read : bool , comp_factor : u8 , stream : Vec < u8 > , read_pos : usize , }
    };
}

ReduceDecoder!();