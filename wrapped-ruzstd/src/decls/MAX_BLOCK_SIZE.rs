macro_rules! MAX_BLOCK_SIZE {
    () => {
        # [doc = " While the spec limits block size to 128KB, the implementation uses"] # [doc = " 128kibibytes"] # [doc = ""] # [doc = " <https://github.com/facebook/zstd/blob/eca205fc7849a61ab287492931a04960ac58e031/doc/educational_decoder/zstd_decompress.c#L28-L29>"] pub const MAX_BLOCK_SIZE : u32 = 128 * 1024 ;
    };
}

MAX_BLOCK_SIZE!();