macro_rules! private_key {
    () => {
        # [cfg (feature = "der")] mod private_key ;
    };
}

private_key!()