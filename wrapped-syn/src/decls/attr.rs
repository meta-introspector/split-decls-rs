macro_rules! attr {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod attr ;
    };
}

attr!()