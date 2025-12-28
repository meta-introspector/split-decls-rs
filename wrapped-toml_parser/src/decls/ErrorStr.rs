macro_rules! ErrorStr {
    () => {
        # [cfg (not (feature = "alloc"))] type ErrorStr = & 'static str ;
    };
}

ErrorStr!();