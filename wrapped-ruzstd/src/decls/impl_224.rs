macro_rules! deps {
    () => {
        BlockType!();
        Write!();
        BlockHeader!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl BlockHeader { # [doc = " Write encoded binary representation of this header into the provided buffer."] pub fn serialize (self , output : & mut Vec < u8 >) { vprintln ! ("Serializing block with the header: {self:?}") ; let encoded_block_type = match self . block_type { BlockType :: Raw => 0 , BlockType :: RLE => 1 , BlockType :: Compressed => 2 , BlockType :: Reserved => panic ! ("You cannot use a reserved block type") , } ; let mut block_header = self . block_size << 3 ; block_header |= encoded_block_type << 1 ; block_header |= self . last_block as u32 ; output . extend_from_slice (& block_header . to_le_bytes () [0 .. 3]) ; } }
    };
}

impl_224!()