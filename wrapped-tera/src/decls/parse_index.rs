macro_rules! parse_index {
    () => {
        # [doc = " serde jsons parse_index"] # [inline] fn parse_index (s : & str) -> Option < usize > { if s . starts_with ('+') || (s . starts_with ('0') && s . len () != 1) { return None ; } s . parse () . ok () }
    };
}

parse_index!()