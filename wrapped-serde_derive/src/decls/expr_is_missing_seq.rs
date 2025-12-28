macro_rules! deps {
    () => {
        Field!();
        Default!();
        Container!();
    };
}

macro_rules! expr_is_missing_seq {
    () => {
        deps!();
        fn expr_is_missing_seq (assign_to : Option < TokenStream > , index : usize , field : & Field , cattrs : & attr :: Container , expecting : & str ,) -> TokenStream { match field . attrs . default () { attr :: Default :: Default => { let span = field . original . span () ; return quote_spanned ! (span => # assign_to _serde ::# private :: Default :: default ()) ; } attr :: Default :: Path (path) => { return quote_spanned ! (path . span () => # assign_to # path ()) ; } attr :: Default :: None => { } } match * cattrs . default () { attr :: Default :: Default | attr :: Default :: Path (_) => { let member = & field . member ; quote ! (# assign_to __default .# member) } attr :: Default :: None => quote ! (return _serde ::# private :: Err (_serde :: de :: Error :: invalid_length (# index , &# expecting))) , } }
    };
}

expr_is_missing_seq!();