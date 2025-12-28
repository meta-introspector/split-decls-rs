macro_rules! T_LAST {
    () => {
        const T_LAST : u32 = T_BASE + T_COUNT - 1 ;
    };
}

T_LAST!();