macro_rules! lsb {
    () => {
        # [doc = " Get the n least significant bits of x."] fn lsb (x : u8 , n : u8) -> u8 { if n >= 8 { return x ; } x & ((1 << n) - 1) }
    };
}

lsb!();