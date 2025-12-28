macro_rules! StringLayer2 {
    () => {
        struct StringLayer2 (& 'static str) ;
    };
}

StringLayer2!()