macro_rules! deps {
    () => {
        ParseStream!();
        ParseNestedMeta!();
        Result!();
    };
}

macro_rules! parse_nested_meta {
    () => {
        deps!();
        pub (crate) fn parse_nested_meta (input : ParseStream , mut logic : impl FnMut (ParseNestedMeta) -> Result < () > ,) -> Result < () > { loop { let path = input . call (parse_meta_path) ? ; logic (ParseNestedMeta { path , input }) ? ; if input . is_empty () { return Ok (()) ; } input . parse :: < Token ! [,] > () ? ; if input . is_empty () { return Ok (()) ; } } }
    };
}

parse_nested_meta!()