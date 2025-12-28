macro_rules! deps {
    () => {
        AllowMultipleAlternatives!();
    };
}

macro_rules! parse_suggestion_values {
    () => {
        deps!();
        fn parse_suggestion_values (nested : ParseNestedMeta < '_ > , allow_multiple : AllowMultipleAlternatives ,) -> syn :: Result < Vec < LitStr > > { let values = if let Ok (val) = nested . value () { vec ! [val . parse () ?] } else { let content ; parenthesized ! (content in nested . input) ; if let AllowMultipleAlternatives :: No = allow_multiple { span_err (nested . input . span () . unwrap () , "expected exactly one string literal for `code = ...`" ,) . emit () ; vec ! [] } else { let literals = Punctuated :: < LitStr , Token ! [,] > :: parse_terminated (& content) ; match literals { Ok (p) if p . is_empty () => { span_err (content . span () . unwrap () , "expected at least one string literal for `code(...)`" ,) . emit () ; vec ! [] } Ok (p) => p . into_iter () . collect () , Err (_) => { span_err (content . span () . unwrap () , "`code(...)` must contain only string literals" ,) . emit () ; vec ! [] } } } } ; Ok (values) }
    };
}

parse_suggestion_values!();