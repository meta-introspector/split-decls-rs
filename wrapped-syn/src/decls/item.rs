macro_rules! item {
    () => {
        # [cfg (feature = "full")] mod item ;
    };
}

item!()