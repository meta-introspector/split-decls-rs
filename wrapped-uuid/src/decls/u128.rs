macro_rules! u128 {
    () => {
        pub (crate) fn u128 () -> u128 { imp :: RngImp :: u128 () }
    };
}

u128!()