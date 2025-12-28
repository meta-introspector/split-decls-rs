macro_rules! StringLayer3 {
    () => {
        struct StringLayer3 (& 'static str) ;
    };
}

StringLayer3!();