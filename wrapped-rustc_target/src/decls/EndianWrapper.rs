macro_rules! EndianWrapper {
    () => {
        # [doc = " `Endian` is in `rustc_abi`, which doesn't have access to the macro and serde."] struct EndianWrapper (rustc_abi :: Endian) ;
    };
}

EndianWrapper!();