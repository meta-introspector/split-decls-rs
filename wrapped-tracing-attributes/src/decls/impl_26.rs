macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Parse for Level { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let _ = input . parse :: < kw :: level > () ? ; let _ = input . parse :: < Token ! [=] > () ? ; let lookahead = input . lookahead1 () ; if lookahead . peek (LitStr) { let str : LitStr = input . parse () ? ; match str . value () { s if s . eq_ignore_ascii_case ("trace") => Ok (Level :: Trace) , s if s . eq_ignore_ascii_case ("debug") => Ok (Level :: Debug) , s if s . eq_ignore_ascii_case ("info") => Ok (Level :: Info) , s if s . eq_ignore_ascii_case ("warn") => Ok (Level :: Warn) , s if s . eq_ignore_ascii_case ("error") => Ok (Level :: Error) , _ => Err (input . error ("unknown verbosity level, expected one of \"trace\", \
                     \"debug\", \"info\", \"warn\", or \"error\", or a number 1-5" ,)) , } } else if lookahead . peek (LitInt) { fn is_level (lit : & LitInt , expected : u64) -> bool { match lit . base10_parse :: < u64 > () { Ok (value) => value == expected , Err (_) => false , } } let int : LitInt = input . parse () ? ; match & int { i if is_level (i , 1) => Ok (Level :: Trace) , i if is_level (i , 2) => Ok (Level :: Debug) , i if is_level (i , 3) => Ok (Level :: Info) , i if is_level (i , 4) => Ok (Level :: Warn) , i if is_level (i , 5) => Ok (Level :: Error) , _ => Err (input . error ("unknown verbosity level, expected one of \"trace\", \
                     \"debug\", \"info\", \"warn\", or \"error\", or a number 1-5" ,)) , } } else if lookahead . peek (Ident) { Ok (Self :: Path (input . parse () ?)) } else { Err (lookahead . error ()) } } }
    };
}

impl_26!();