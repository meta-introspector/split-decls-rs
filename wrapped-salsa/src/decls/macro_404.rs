macro_rules! macro_404 {
    () => {
        # [cfg (feature = "compact_str")] fallback_impl ! { compact_str :: CompactString , }
    };
}

macro_404!();