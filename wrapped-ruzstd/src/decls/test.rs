macro_rules! deps {
    () => {
        BitReaderReversed!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { # [test] fn it_works () { let data = [0b10101010 , 0b01010101] ; let mut br = super :: BitReaderReversed :: new (& data) ; assert_eq ! (br . get_bits (1) , 0) ; assert_eq ! (br . get_bits (1) , 1) ; assert_eq ! (br . get_bits (1) , 0) ; assert_eq ! (br . get_bits (4) , 0b1010) ; assert_eq ! (br . get_bits (4) , 0b1101) ; assert_eq ! (br . get_bits (4) , 0b0101) ; assert_eq ! (br . get_bits (4) , 0b0000) ; assert_eq ! (br . get_bits (4) , 0b0000) ; assert_eq ! (br . bits_remaining () , - 7) ; } }
    };
}

test!()