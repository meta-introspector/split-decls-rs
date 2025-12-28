macro_rules! parse_macro_input {
    () => {
        # [cfg (all (feature = "parsing" , feature = "proc-macro"))] mod parse_macro_input ;
    };
}

parse_macro_input!()