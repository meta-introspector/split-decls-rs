macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfd {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is in NFD."] # [inline] pub fn is_nfd (s : & str) -> bool { match is_nfd_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . nfd ()) , } }
    };
}

is_nfd!();