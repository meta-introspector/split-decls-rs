macro_rules! reverse_lsb {
    () => {
        # [doc = " Reverse the n least significant bits of x."] # [doc = " The (16 - n) most significant bits of the result will be zero."] pub fn reverse_lsb (x : u16 , n : usize) -> u16 { debug_assert ! (n > 0) ; debug_assert ! (n <= 16) ; x . reverse_bits () >> (16 - n) }
    };
}

reverse_lsb!()