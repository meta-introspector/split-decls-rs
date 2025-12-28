macro_rules! canonical_fully_decomposed {
    () => {
        pub (crate) fn canonical_fully_decomposed (c : char) -> Option < & 'static [char] > { mph_lookup (c . into () , CANONICAL_DECOMPOSED_SALT , CANONICAL_DECOMPOSED_KV , pair_lookup_fk , pair_lookup_fv_opt , None ,) . map (| (start , len) | & CANONICAL_DECOMPOSED_CHARS [start as usize ..] [.. len as usize]) }
    };
}

canonical_fully_decomposed!()