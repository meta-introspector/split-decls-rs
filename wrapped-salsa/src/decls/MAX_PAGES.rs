macro_rules! MAX_PAGES {
    () => {
        const MAX_PAGES : usize = 1 << (u32 :: BITS as usize - PAGE_LEN_BITS) ;
    };
}

MAX_PAGES!();