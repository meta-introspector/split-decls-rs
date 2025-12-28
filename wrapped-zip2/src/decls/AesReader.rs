macro_rules! deps {
    () => {
        AesMode!();
    };
}

macro_rules! AesReader {
    () => {
        deps!();
        pub struct AesReader < R > { reader : R , aes_mode : AesMode , data_length : u64 , }
    };
}

AesReader!();