macro_rules! op {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod op ;
    };
}

op!()