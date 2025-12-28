macro_rules! BlockDecodingStrategy {
    () => {
        pub enum BlockDecodingStrategy { All , UptoBlocks (usize) , UptoBytes (usize) , }
    };
}

BlockDecodingStrategy!();