macro_rules! deps {
    () => {
        Stmts!();
        Container!();
        Parameters!();
    };
}

macro_rules! deserialize_in_place_body {
    () => {
        deps!();
        # [cfg (not (feature = "deserialize_in_place"))] fn deserialize_in_place_body (_cont : & Container , _params : & Parameters) -> Option < Stmts > { None }
    };
}

deserialize_in_place_body!();