macro_rules! TokensOrDefault {
    () => {
        pub (crate) struct TokensOrDefault < 'a , T : 'a > (pub & 'a Option < T >) ;
    };
}

TokensOrDefault!()