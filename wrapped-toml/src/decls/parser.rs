macro_rules! parser {
    () => {
        # [cfg (feature = "parse")] mod parser ;
    };
}

parser!();