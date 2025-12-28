macro_rules! deps {
    () => {
        RawString!();
        Key!();
    };
}

macro_rules! parse_key_path {
    () => {
        deps!();
        pub (crate) fn parse_key_path (source : toml_parser :: Source < '_ > , errors : & mut dyn prelude :: ErrorSink ,) -> Vec < crate :: Key > { let tokens = source . lex () . into_vec () ; let mut events = Vec :: with_capacity (tokens . len ()) ; let mut receiver = ValidateWhitespace :: new (& mut events , source) ; # [cfg (not (feature = "unbounded"))] let mut receiver = RecursionGuard :: new (& mut receiver , LIMIT) ; # [cfg (not (feature = "unbounded"))] let receiver = & mut receiver ; # [cfg (feature = "unbounded")] let receiver = & mut receiver ; toml_parser :: parser :: parse_key (& tokens , receiver , errors) ; let mut input = prelude :: Input :: new (& events) ; let mut prefix = None ; let mut path = None ; let mut key = None ; let mut suffix = None ; while let Some (event) = input . next_token () { match event . kind () { toml_parser :: parser :: EventKind :: Whitespace => { let raw = RawString :: with_span (event . span () . start () .. event . span () . end ()) ; if prefix . is_none () { prefix = Some (raw) ; } else if suffix . is_none () { suffix = Some (raw) ; } } _ => { let (local_path , local_key) = key :: on_key (event , & mut input , source , errors) ; path = Some (local_path) ; key = local_key ; } } } if let Some (mut key) = key { if let Some (prefix) = prefix { key . leaf_decor . set_prefix (prefix) ; } if let Some (suffix) = suffix { key . leaf_decor . set_suffix (suffix) ; } let mut path = path . unwrap_or_default () ; path . push (key) ; path } else { Default :: default () } }
    };
}

parse_key_path!();