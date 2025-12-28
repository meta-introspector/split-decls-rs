macro_rules! path {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod path ;
    };
}

path!()