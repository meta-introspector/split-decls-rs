macro_rules! V_LAST {
    () => {
        const V_LAST : u32 = V_BASE + V_COUNT - 1 ;
    };
}

V_LAST!();