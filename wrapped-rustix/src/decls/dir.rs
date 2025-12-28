macro_rules! dir {
    () => {
        # [cfg (all (feature = "alloc" , not (any (target_os = "espidf" , target_os = "redox"))))] mod dir ;
    };
}

dir!();