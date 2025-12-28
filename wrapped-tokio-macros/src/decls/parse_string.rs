macro_rules! parse_string {
    () => {
        fn parse_string (int : syn :: Lit , span : Span , field : & str) -> Result < String , syn :: Error > { match int { syn :: Lit :: Str (s) => Ok (s . value ()) , syn :: Lit :: Verbatim (s) => Ok (s . to_string ()) , _ => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as string.") ,)) , } }
    };
}

parse_string!()