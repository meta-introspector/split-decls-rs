macro_rules! parse_quote {
    () => {
        # [cfg (all (feature = "parsing" , feature = "printing"))] mod parse_quote ;
    };
}

parse_quote!();