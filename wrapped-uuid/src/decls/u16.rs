macro_rules! u16 {
    () => {
        pub (crate) fn u16 () -> u16 { imp :: RngImp :: u16 () }
    };
}

u16!();