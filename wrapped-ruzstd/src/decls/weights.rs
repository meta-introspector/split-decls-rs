macro_rules! deps {
    () => {
        HuffmanTable!();
    };
}

macro_rules! weights {
    () => {
        deps!();
        # [test] fn weights () { for amount in 2 ..= 256 { let mut weights = distribute_weights (amount) ; assert_eq ! (weights . len () , amount) ; let sum = weights . iter () . copied () . map (| weight | 1 << weight) . sum :: < usize > () ; assert ! (sum . is_power_of_two ()) ; for num_bit_limit in (amount . ilog2 () as usize + 1) ..= 11 { redistribute_weights (& mut weights , num_bit_limit) ; let sum = weights . iter () . copied () . map (| weight | 1 << weight) . sum :: < usize > () ; assert ! (sum . is_power_of_two ()) ; assert ! (sum . ilog2 () <= 11 , "Max bits too big: sum: {} {weights:?}" , sum) ; let codes = HuffmanTable :: build_from_weights (& weights) . codes ; for (code , num_bits) in codes . iter () . copied () { for (code2 , num_bits2) in codes . iter () . copied () { if num_bits == 0 || num_bits2 == 0 || (code , num_bits) == (code2 , num_bits2) { continue ; } if num_bits <= num_bits2 { let code2_shifted = code2 >> (num_bits2 - num_bits) ; assert_ne ! (code , code2_shifted , "{code:b},{num_bits:} is prefix of {code2:b},{num_bits2:}") ; } } } } } }
    };
}

weights!()