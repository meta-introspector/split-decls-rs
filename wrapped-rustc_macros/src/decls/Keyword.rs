macro_rules! Keyword {
    () => {
        struct Keyword { name : Ident , value : LitStr , }
    };
}

Keyword!()