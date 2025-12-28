macro_rules! BigInt {
    () => {
        pub (crate) struct BigInt { digits : Vec < u8 > , }
    };
}

BigInt!();