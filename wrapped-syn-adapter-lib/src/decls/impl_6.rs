macro_rules! deps {
    () => {
        SynAdapter!();
        LibSynAdapter!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (feature = "syn-parsing")] impl SynAdapter for LibSynAdapter { fn parse_file (& self , path : & Path) -> Result < File > { println ! ("[LibSynAdapter] Parsing file: {:?}" , path) ; let content = fs :: read_to_string (path) . context (format ! ("Failed to read file {:?}" , path)) ? ; syn :: parse_file (& content) . context (format ! ("Failed to parse Rust file {:?}" , path)) } fn parse_str (& self , code : & str) -> Result < File > { println ! ("[LibSynAdapter] Parsing string: {}" , code) ; syn :: parse_file (code) . context ("Failed to parse Rust string") } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_6!();