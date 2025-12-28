macro_rules! verbatim {
    () => {
        # [cfg (all (any (feature = "full" , feature = "derive") , feature = "parsing"))] mod verbatim ;
    };
}

verbatim!()