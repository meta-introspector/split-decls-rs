macro_rules! restriction {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod restriction ;
    };
}

restriction!()