macro_rules! deps {
    () => {
        BitWriter!();
    };
}

macro_rules! raw_literals {
    () => {
        deps!();
        fn raw_literals (literals : & [u8] , writer : & mut BitWriter < & mut Vec < u8 > >) { writer . write_bits (0u8 , 2) ; writer . write_bits (0b11u8 , 2) ; writer . write_bits (literals . len () as u32 , 20) ; writer . append_bytes (literals) ; }
    };
}

raw_literals!()