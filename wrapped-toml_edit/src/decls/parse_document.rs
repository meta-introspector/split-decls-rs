macro_rules! deps {
    () => {
        Document!();
    };
}

macro_rules! parse_document {
    () => {
        deps!();
        pub (crate) fn parse_document < 's > (source : toml_parser :: Source < 's > , errors : & mut dyn prelude :: ErrorSink ,) -> crate :: Document < & 's str > { let tokens = source . lex () . into_vec () ; let mut events = Vec :: with_capacity (tokens . len ()) ; let mut receiver = ValidateWhitespace :: new (& mut events , source) ; # [cfg (not (feature = "unbounded"))] let mut receiver = RecursionGuard :: new (& mut receiver , LIMIT) ; # [cfg (not (feature = "unbounded"))] let receiver = & mut receiver ; # [cfg (feature = "unbounded")] let receiver = & mut receiver ; toml_parser :: parser :: parse_document (& tokens , receiver , errors) ; let mut input = prelude :: Input :: new (& events) ; let doc = document :: document (& mut input , source , errors) ; doc }
    };
}

parse_document!()