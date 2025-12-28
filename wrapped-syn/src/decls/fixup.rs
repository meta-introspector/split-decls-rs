macro_rules! fixup {
    () => {
        # [cfg (all (any (feature = "full" , feature = "derive") , feature = "printing"))] mod fixup ;
    };
}

fixup!()