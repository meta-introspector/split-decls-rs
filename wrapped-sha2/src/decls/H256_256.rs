macro_rules! deps {
    () => {
        State256!();
    };
}

macro_rules! H256_256 {
    () => {
        deps!();
        pub (crate) const H256_256 : State256 = [0x6a09e667 , 0xbb67ae85 , 0x3c6ef372 , 0xa54ff53a , 0x510e527f , 0x9b05688c , 0x1f83d9ab , 0x5be0cd19 ,] ;
    };
}

H256_256!()