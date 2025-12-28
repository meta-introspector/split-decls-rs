macro_rules! to_u64s {
    () => {
        # [inline (always)] # [allow (dead_code)] fn to_u64s (block : & [u8 ; 128]) -> [u64 ; 16] { core :: array :: from_fn (| i | { let chunk = block [8 * i ..] [.. 8] . try_into () . unwrap () ; u64 :: from_be_bytes (chunk) }) }
    };
}

to_u64s!();