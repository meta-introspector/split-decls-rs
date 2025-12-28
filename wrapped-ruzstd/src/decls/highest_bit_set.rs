macro_rules! highest_bit_set {
    () => {
        # [doc = " Assert that the provided value is greater than zero, and returns index of the first set bit"] fn highest_bit_set (x : usize) -> usize { assert ! (x > 0) ; usize :: BITS as usize - x . leading_zeros () as usize }
    };
}

highest_bit_set!();