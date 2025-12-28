macro_rules! impl_241_plus_bytes {
    () => {
        # [inline] fn impl_241_plus_bytes (secret : & Secret , input : & [u8]) -> u128 { assert_input_range ! (241 .., input . len ()) ; dispatch ! { fn oneshot_impl <> (secret : & Secret , input : & [u8]) -> u128 [] } }
    };
}

impl_241_plus_bytes!()