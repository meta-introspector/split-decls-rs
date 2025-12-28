macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfkc_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is in NFKC."] # [inline] pub fn is_nfkc_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfkc , false) }
    };
}

is_nfkc_quick!()