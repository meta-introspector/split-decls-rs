macro_rules! print {
    () => {
        # [cfg (all (any (feature = "full" , feature = "derive") , feature = "printing"))] mod print ;
    };
}

print!()