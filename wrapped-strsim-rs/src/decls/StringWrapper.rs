macro_rules! StringWrapper {
    () => {
        struct StringWrapper < 'a > (& 'a str) ;
    };
}

StringWrapper!();