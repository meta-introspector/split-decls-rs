macro_rules! deps {
    () => {
        EventArgs!();
        StrArg!();
        InstrumentArgs!();
        ExprArg!();
        Skips!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Parse for InstrumentArgs { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut args = Self :: default () ; while ! input . is_empty () { let lookahead = input . lookahead1 () ; if lookahead . peek (kw :: name) { if args . name . is_some () { return Err (input . error ("expected only a single `name` argument")) ; } let name = input . parse :: < StrArg < kw :: name > > () ? . value ; args . name = Some (name) ; } else if lookahead . peek (LitStr) { if args . name . is_some () { return Err (input . error ("expected only a single `name` argument")) ; } args . name = Some (input . parse () ?) ; } else if lookahead . peek (kw :: target) { if args . target . is_some () { return Err (input . error ("expected only a single `target` argument")) ; } let target = input . parse :: < StrArg < kw :: target > > () ? . value ; args . target = Some (target) ; } else if lookahead . peek (kw :: parent) { if args . target . is_some () { return Err (input . error ("expected only a single `parent` argument")) ; } let parent = input . parse :: < ExprArg < kw :: parent > > () ? ; args . parent = Some (parent . value) ; } else if lookahead . peek (kw :: follows_from) { if args . target . is_some () { return Err (input . error ("expected only a single `follows_from` argument")) ; } let follows_from = input . parse :: < ExprArg < kw :: follows_from > > () ? ; args . follows_from = Some (follows_from . value) ; } else if lookahead . peek (kw :: level) { if args . level . is_some () { return Err (input . error ("expected only a single `level` argument")) ; } args . level = Some (input . parse () ?) ; } else if lookahead . peek (kw :: skip) { if ! args . skips . is_empty () { return Err (input . error ("expected only a single `skip` argument")) ; } if args . skip_all { return Err (input . error ("expected either `skip` or `skip_all` argument")) ; } let Skips (skips) = input . parse () ? ; args . skips = skips ; } else if lookahead . peek (kw :: skip_all) { if args . skip_all { return Err (input . error ("expected only a single `skip_all` argument")) ; } if ! args . skips . is_empty () { return Err (input . error ("expected either `skip` or `skip_all` argument")) ; } let _ = input . parse :: < kw :: skip_all > () ? ; args . skip_all = true ; } else if lookahead . peek (kw :: fields) { if args . fields . is_some () { return Err (input . error ("expected only a single `fields` argument")) ; } args . fields = Some (input . parse () ?) ; } else if lookahead . peek (kw :: err) { let _ = input . parse :: < kw :: err > () ; let err_args = EventArgs :: parse (input) ? ; args . err_args = Some (err_args) ; } else if lookahead . peek (kw :: ret) { let _ = input . parse :: < kw :: ret > () ? ; let ret_args = EventArgs :: parse (input) ? ; args . ret_args = Some (ret_args) ; } else if lookahead . peek (Token ! [,]) { let _ = input . parse :: < Token ! [,] > () ? ; } else { args . parse_warnings . push (lookahead . error ()) ; let _ = input . parse :: < proc_macro2 :: TokenTree > () ; } } Ok (args) } }
    };
}

impl_4!();