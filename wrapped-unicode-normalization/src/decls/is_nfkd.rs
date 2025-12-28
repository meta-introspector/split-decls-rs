macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfkd {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is in NFKD."] # [inline] pub fn is_nfkd (s : & str) -> bool { match is_nfkd_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfkd ()) , } }
    };
}

is_nfkd!()