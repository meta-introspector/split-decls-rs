macro_rules! expr {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod expr ;
    };
}

expr!()