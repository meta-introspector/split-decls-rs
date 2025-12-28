macro_rules! properties {
    () => {
        # [cfg (not (loom))] mod properties ;
    };
}

properties!()