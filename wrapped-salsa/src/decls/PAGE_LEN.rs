macro_rules! PAGE_LEN {
    () => {
        const PAGE_LEN : usize = 1 << PAGE_LEN_BITS ;
    };
}

PAGE_LEN!();