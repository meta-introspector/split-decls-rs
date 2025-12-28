macro_rules! u16_hash {
    () => {
        # [inline] const fn u16_hash (hash : u64) -> u16 { const K : u16 = 0xecc5 ; let (part1 , part2 , part3 , part4) = (hash as u16 , (hash >> 16) as u16 , (hash >> 32) as u16 , (hash >> 48) as u16) ; part1 . wrapping_add (part2) . wrapping_mul (K) . wrapping_add (part3) . wrapping_mul (K) . wrapping_add (part4) . wrapping_mul (K) }
    };
}

u16_hash!()