macro_rules! deps {
    () => {
        Container!();
        Field!();
        Expr!();
        Fragment!();
        Default!();
    };
}

macro_rules! expr_is_missing {
    () => {
        deps!();
        fn expr_is_missing (field : & Field , cattrs : & attr :: Container) -> Fragment { match field . attrs . default () { attr :: Default :: Default => { let span = field . original . span () ; let func = quote_spanned ! (span => _serde ::# private :: Default :: default) ; return quote_expr ! (# func ()) ; } attr :: Default :: Path (path) => { return Fragment :: Expr (quote_spanned ! (path . span () => # path ())) ; } attr :: Default :: None => { } } match * cattrs . default () { attr :: Default :: Default | attr :: Default :: Path (_) => { let member = & field . member ; return quote_expr ! (__default .# member) ; } attr :: Default :: None => { } } let name = field . attrs . name () . deserialize_name () ; match field . attrs . deserialize_with () { None => { let span = field . original . span () ; let func = quote_spanned ! (span => _serde ::# private :: de :: missing_field) ; quote_expr ! { # func (# name) ? } } Some (_) => { quote_expr ! { return _serde ::# private :: Err (< __A :: Error as _serde :: de :: Error >:: missing_field (# name)) } } } }
    };
}

expr_is_missing!()