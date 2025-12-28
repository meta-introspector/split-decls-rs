macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! string_roundtrip {
    () => {
        deps!();
        # [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn string_roundtrip () { Value :: from ("hello") . to_string () . parse :: < Value > () . unwrap () ; }
    };
}

string_roundtrip!();