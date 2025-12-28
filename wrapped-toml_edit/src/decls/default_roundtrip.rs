macro_rules! deps {
    () => {
        DocumentMut!();
    };
}

macro_rules! default_roundtrip {
    () => {
        deps!();
        # [test] # [cfg (feature = "parse")] # [cfg (feature = "display")] fn default_roundtrip () { DocumentMut :: default () . to_string () . parse :: < DocumentMut > () . unwrap () ; }
    };
}

default_roundtrip!()