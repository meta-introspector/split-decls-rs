macro_rules! deps {
    () => {
        MockSynAdapter!();
        SynAdapter!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl SynAdapter for MockSynAdapter { fn parse_file (& self , path : & Path) -> Result < File > { println ! ("[MockSynAdapter] Mock parsing file: {:?}" , path) ; Ok (File { shebang : None , attrs : vec ! [] , items : vec ! [] , }) } fn parse_str (& self , code : & str) -> Result < File > { println ! ("[MockSynAdapter] Mock parsing string: {}" , code) ; Ok (File { shebang : None , attrs : vec ! [] , items : vec ! [] , }) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_3!()