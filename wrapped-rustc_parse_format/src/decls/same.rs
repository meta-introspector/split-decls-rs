macro_rules! deps {
    () => {
        Piece!();
        Parser!();
        ParseMode!();
    };
}

macro_rules! same {
    () => {
        deps!();
        # [track_caller] fn same (fmt : & 'static str , p : & [Piece < 'static >]) { let parser = Parser :: new (fmt , None , None , false , ParseMode :: Format) ; assert_eq ! (parser . collect ::< Vec < Piece <'static >>> () , p) ; }
    };
}

same!();