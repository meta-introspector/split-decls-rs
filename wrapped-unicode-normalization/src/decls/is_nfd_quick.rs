macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfd_quick {
    () => {
        deps!();
        # [doc = " Quickly check if a string is in NFD."] # [inline] pub fn is_nfd_quick < I : Iterator < Item = char > > (s : I) -> IsNormalized { quick_check (s , tables :: qc_nfd , false) }
    };
}

is_nfd_quick!();