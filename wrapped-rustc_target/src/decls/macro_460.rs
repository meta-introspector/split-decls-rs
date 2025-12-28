macro_rules! deps {
    () => {
        LinkerFlavorCli!();
    };
}

macro_rules! macro_460 {
    () => {
        deps!();
        crate :: json :: serde_deserialize_from_str ! (LinkerFlavorCli) ;
    };
}

macro_460!();