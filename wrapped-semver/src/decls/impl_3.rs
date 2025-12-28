macro_rules! deps {
    () => {
        Comparator!();
        Op!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Display for Comparator { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let op = match self . op { Op :: Exact => "=" , Op :: Greater => ">" , Op :: GreaterEq => ">=" , Op :: Less => "<" , Op :: LessEq => "<=" , Op :: Tilde => "~" , Op :: Caret => "^" , Op :: Wildcard => "" , } ; formatter . write_str (op) ? ; write ! (formatter , "{}" , self . major) ? ; if let Some (minor) = & self . minor { write ! (formatter , ".{}" , minor) ? ; if let Some (patch) = & self . patch { write ! (formatter , ".{}" , patch) ? ; if ! self . pre . is_empty () { write ! (formatter , "-{}" , self . pre) ? ; } } else if self . op == Op :: Wildcard { formatter . write_str (".*") ? ; } } else if self . op == Op :: Wildcard { formatter . write_str (".*") ? ; } Ok (()) } }
    };
}

impl_3!();