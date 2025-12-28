macro_rules! u64 {
    () => {
        pub (crate) fn u64 () -> u64 { imp :: RngImp :: u64 () }
    };
}

u64!()