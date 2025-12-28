macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfkd_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is in NFKD."] # [inline] pub fn is_nfkd_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfkd , false) }
    };
}

is_nfkd_quick!()