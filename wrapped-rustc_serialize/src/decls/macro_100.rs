macro_rules! macro_100 {
    () => {
        impl_write_signed_leb128 ! (write_i16_leb128 , i16) ;
    };
}

macro_100!()