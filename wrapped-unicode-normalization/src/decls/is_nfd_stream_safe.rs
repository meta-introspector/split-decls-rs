macro_rules! deps {
    () => {
        IsNormalized!();
    };
}

macro_rules! is_nfd_stream_safe {
    () => {
        deps!();
        # [doc = " Authoritatively check if a string is Stream-Safe NFD."] # [inline] pub fn is_nfd_stream_safe (s : & str) -> bool { match is_nfd_stream_safe_quick (s . chars ()) { IsNormalized :: Yes => true , IsNormalized :: No => false , IsNormalized :: Maybe => s . chars () . eq (s . chars () . stream_safe () . nfd ()) , } }
    };
}

is_nfd_stream_safe!()