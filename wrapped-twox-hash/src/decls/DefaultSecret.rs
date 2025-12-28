macro_rules! DefaultSecret {
    () => {
        pub type DefaultSecret = [u8 ; DEFAULT_SECRET_LENGTH] ;
    };
}

DefaultSecret!();