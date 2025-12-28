macro_rules! whitespace {
    () => {
        # [cfg (all (feature = "parsing" , feature = "full"))] mod whitespace ;
    };
}

whitespace!();