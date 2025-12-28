macro_rules! derive {
    () => {
        # [cfg (any (feature = "full" , feature = "derive"))] mod derive ;
    };
}

derive!();