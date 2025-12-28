macro_rules! PAGE_LEN_MASK {
    () => {
        const PAGE_LEN_MASK : usize = PAGE_LEN - 1 ;
    };
}

PAGE_LEN_MASK!()