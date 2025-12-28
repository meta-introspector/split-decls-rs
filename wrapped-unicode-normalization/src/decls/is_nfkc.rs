macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfkc {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is in NFKC."] # [inline] pub fn is_nfkc (s : & str) -> bool { match is_nfkc_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfkc ()) , } }
    };
}

is_nfkc!()