macro_rules! getpath {
    () => {
        # [cfg (all (apple , feature = "alloc"))] mod getpath ;
    };
}

getpath!();