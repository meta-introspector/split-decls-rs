macro_rules! Filters {
    () => {
        # [doc = " A custom chain of filters to configure an encoding stream."] pub struct Filters { inner : Vec < lzma_sys :: lzma_filter > , lzma_opts : LinkedList < lzma_sys :: lzma_options_lzma > , }
    };
}

Filters!();