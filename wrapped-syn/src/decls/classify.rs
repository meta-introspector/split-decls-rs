macro_rules! classify {
    () => {
        # [cfg (any (all (feature = "parsing" , feature = "full") , all (feature = "printing" , any (feature = "full" , feature = "derive")) ,))] mod classify ;
    };
}

classify!()