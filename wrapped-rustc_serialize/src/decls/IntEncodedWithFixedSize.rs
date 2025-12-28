macro_rules! IntEncodedWithFixedSize {
    () => {
        # [doc = " An integer that will always encode to 8 bytes."] pub struct IntEncodedWithFixedSize (pub u64) ;
    };
}

IntEncodedWithFixedSize!();