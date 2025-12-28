macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfd_stream_safe_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is Stream-Safe NFD."] # [inline] pub fn is_nfd_stream_safe_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfd , true) }
    };
}

is_nfd_stream_safe_quick!()