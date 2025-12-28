macro_rules! deps {
    () => {
        IntEncodedWithFixedSize!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl IntEncodedWithFixedSize { pub const ENCODED_SIZE : usize = 8 ; }
    };
}

impl_129!()