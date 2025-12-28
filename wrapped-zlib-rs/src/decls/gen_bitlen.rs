macro_rules! deps {
    () => {
        State!();
        Heap!();
        TreeDesc!();
    };
}

macro_rules! gen_bitlen {
    () => {
        deps!();
        fn gen_bitlen < const N : usize > (state : & mut State , heap : & mut Heap , desc : & mut TreeDesc < N > ,) -> [u16 ; MAX_BITS + 1] { let tree = & mut desc . dyn_tree ; let max_code = desc . max_code ; let stree = desc . stat_desc . static_tree ; let extra = desc . stat_desc . extra_bits ; let base = desc . stat_desc . extra_base ; let max_length = desc . stat_desc . max_length ; let mut bl_count = [0u16 ; MAX_BITS + 1] ; * tree [heap . heap [heap . heap_max] as usize] . len_mut () = 0 ; let mut overflow : i32 = 0 ; for h in heap . heap_max + 1 .. HEAP_SIZE { let n = heap . heap [h] as usize ; let mut bits = tree [tree [n] . dad () as usize] . len () + 1 ; if bits > max_length { bits = max_length ; overflow += 1 ; } * tree [n] . len_mut () = bits ; if n > max_code { continue ; } bl_count [bits as usize] += 1 ; let mut xbits = 0 ; if n >= base { xbits = extra [n - base] as usize ; } let f = tree [n] . freq () as usize ; state . opt_len += f * (bits as usize + xbits) ; if ! stree . is_empty () { state . static_len += f * (stree [n] . len () as usize + xbits) ; } } if overflow == 0 { return bl_count ; } loop { let mut bits = max_length as usize - 1 ; while bl_count [bits] == 0 { bits -= 1 ; } bl_count [bits] -= 1 ; bl_count [bits + 1] += 2 ; bl_count [max_length as usize] -= 1 ; overflow -= 2 ; if overflow <= 0 { break ; } } let mut h = HEAP_SIZE ; for bits in (1 ..= max_length) . rev () { let mut n = bl_count [bits as usize] ; while n != 0 { h -= 1 ; let m = heap . heap [h] as usize ; if m > max_code { continue ; } if tree [m] . len () != bits { state . opt_len += (bits * tree [m] . freq ()) as usize ; state . opt_len -= (tree [m] . len () * tree [m] . freq ()) as usize ; * tree [m] . len_mut () = bits ; } n -= 1 ; } } bl_count }
    };
}

gen_bitlen!();