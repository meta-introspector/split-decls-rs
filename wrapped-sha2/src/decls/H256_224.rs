macro_rules! deps {
    () => {
        State256!();
    };
}

macro_rules! H256_224 {
    () => {
        deps!();
        pub (crate) const H256_224 : State256 = [0xc1059ed8 , 0x367cd507 , 0x3070dd17 , 0xf70e5939 , 0xffc00b31 , 0x68581511 , 0x64f98fa7 , 0xbefa4fa4 ,] ;
    };
}

H256_224!();