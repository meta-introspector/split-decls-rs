macro_rules! BLOCK_SIZE {
    () => {
        const BLOCK_SIZE : usize = < sha1 :: block_api :: Sha1Core as BlockSizeUser > :: BlockSize :: USIZE ;
    };
}

BLOCK_SIZE!();