macro_rules! deps {
    () => {
        ParseMode!();
        Parser!();
    };
}

macro_rules! musterr {
    () => {
        deps!();
        fn musterr (s : & str) { let mut p = Parser :: new (s , None , None , false , ParseMode :: Format) ; p . next () ; assert ! (! p . errors . is_empty ()) ; }
    };
}

musterr!();