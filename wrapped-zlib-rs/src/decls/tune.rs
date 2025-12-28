macro_rules! deps {
    () => {
        ReturnCode!();
        DeflateStream!();
    };
}

macro_rules! tune {
    () => {
        deps!();
        pub fn tune (stream : & mut DeflateStream , good_length : usize , max_lazy : usize , nice_length : usize , max_chain : usize ,) -> ReturnCode { stream . state . good_match = good_length as u16 ; stream . state . max_lazy_match = max_lazy as u16 ; stream . state . nice_match = nice_length as u16 ; stream . state . max_chain_length = max_chain as u16 ; ReturnCode :: Ok }
    };
}

tune!();