macro_rules! mocked {
    () => {
        # [cfg (all (test , loom))] mod mocked ;
    };
}

mocked!()