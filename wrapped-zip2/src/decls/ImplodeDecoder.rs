macro_rules! ImplodeDecoder {
    () => {
        # [derive (Debug)] pub struct ImplodeDecoder < R > { compressed_reader : R , uncompressed_size : u64 , stream_read : bool , large_wnd : bool , lit_tree : bool , stream : Vec < u8 > , read_pos : usize , }
    };
}

ImplodeDecoder!()