macro_rules! zswap32 {
    () => {
        const fn zswap32 (q : u32) -> u32 { u32 :: from_be (q . to_le ()) }
    };
}

zswap32!();