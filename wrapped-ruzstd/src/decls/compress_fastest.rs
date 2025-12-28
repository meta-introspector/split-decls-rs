macro_rules! deps {
    () => {
        BlockType!();
        CompressState!();
        BlockHeader!();
        Matcher!();
    };
}

macro_rules! compress_fastest {
    () => {
        deps!();
        # [doc = " Compresses a single block at [`crate::encoding::CompressionLevel::Fastest`]."] # [doc = ""] # [doc = " # Parameters"] # [doc = " - `state`: [`CompressState`] so the compressor can refer to data before"] # [doc = "   the start of this block"] # [doc = " - `last_block`: Whether or not this block is going to be the last block in the frame"] # [doc = "   (needed because this info is written into the block header)"] # [doc = " - `uncompressed_data`: A block's worth of uncompressed data, taken from the"] # [doc = "   larger input"] # [doc = " - `output`: As `uncompressed_data` is compressed, it's appended to `output`."] # [inline] pub fn compress_fastest < M : Matcher > (state : & mut CompressState < M > , last_block : bool , uncompressed_data : Vec < u8 > , output : & mut Vec < u8 > ,) { let block_size = uncompressed_data . len () as u32 ; if uncompressed_data . iter () . all (| x | uncompressed_data [0] . eq (x)) { let rle_byte = uncompressed_data [0] ; state . matcher . commit_space (uncompressed_data) ; state . matcher . skip_matching () ; let header = BlockHeader { last_block , block_type : crate :: blocks :: block :: BlockType :: RLE , block_size , } ; header . serialize (output) ; output . push (rle_byte) ; } else { let mut compressed = Vec :: new () ; state . matcher . commit_space (uncompressed_data) ; compress_block (state , & mut compressed) ; if compressed . len () >= MAX_BLOCK_SIZE as usize { let header = BlockHeader { last_block , block_type : crate :: blocks :: block :: BlockType :: Raw , block_size , } ; header . serialize (output) ; output . extend_from_slice (state . matcher . get_last_space ()) ; } else { let header = BlockHeader { last_block , block_type : crate :: blocks :: block :: BlockType :: Compressed , block_size : compressed . len () as u32 , } ; header . serialize (output) ; output . extend (compressed) ; } } }
    };
}

compress_fastest!()