macro_rules! BuildTarget {
    () => {
        # [derive (Deserialize)] pub (crate) struct BuildTarget { pub crate_types : Vec < String > , }
    };
}

BuildTarget!()