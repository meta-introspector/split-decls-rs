macro_rules! LzmaOptions {
    () => {
        # [doc = " Options that can be used to configure how LZMA encoding happens."] # [doc = ""] # [doc = " This builder is consumed by a number of other methods."] pub struct LzmaOptions { raw : lzma_sys :: lzma_options_lzma , }
    };
}

LzmaOptions!();