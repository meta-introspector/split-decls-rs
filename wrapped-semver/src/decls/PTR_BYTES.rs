macro_rules! PTR_BYTES {
    () => {
        const PTR_BYTES : usize = mem :: size_of :: < NonNull < u8 > > () ;
    };
}

PTR_BYTES!()