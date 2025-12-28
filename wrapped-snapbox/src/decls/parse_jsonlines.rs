macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! parse_jsonlines {
    () => {
        deps!();
        # [cfg (feature = "json")] fn parse_jsonlines (text : & str) -> Result < Vec < serde_json :: Value > , serde_json :: Error > { let mut lines = Vec :: new () ; for line in text . lines () { let line = line . trim () ; if line . is_empty () { continue ; } let json = serde_json :: from_str :: < serde_json :: Value > (line) ? ; lines . push (json) ; } Ok (lines) }
    };
}

parse_jsonlines!();