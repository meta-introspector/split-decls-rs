macro_rules! AES_BLOCK_SIZE {
    () => {
        # [doc = " Internal block size of an AES cipher."] const AES_BLOCK_SIZE : usize = 16 ;
    };
}

AES_BLOCK_SIZE!();