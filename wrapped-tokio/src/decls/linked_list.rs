macro_rules! linked_list {
    () => {
        # [cfg (any (feature = "fs" , feature = "net" , feature = "process" , feature = "rt" , feature = "sync" , feature = "signal" , feature = "time" , fuzzing ,))] pub (crate) mod linked_list ;
    };
}

linked_list!();