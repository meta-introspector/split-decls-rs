macro_rules! HexBytes {
    () => {
        struct HexBytes < 'a > (& 'a [u8]) ;
    };
}

HexBytes!();