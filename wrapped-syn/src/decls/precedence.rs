macro_rules! precedence {
    () => {
        # [cfg (all (any (feature = "full" , feature = "derive") , any (feature = "parsing" , feature = "printing")))] mod precedence ;
    };
}

precedence!();