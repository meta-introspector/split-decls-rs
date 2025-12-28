macro_rules! deps {
    () => {
        Piece!();
        Parser!();
        ParseMode!();
    };
}

macro_rules! format_open_brace_tab {
    () => {
        deps!();
        # [test] fn format_open_brace_tab () { let fmt_pre = r###""{\t""### ; let fmt = "{\t" ; let mut parser = Parser :: new (fmt , None , Some (fmt_pre . into ()) , false , ParseMode :: Format) ; let _ = parser . by_ref () . collect :: < Vec < Piece < 'static > > > () ; assert_eq ! (parser . errors [0] . span , 4 .. 4) ; }
    };
}

format_open_brace_tab!()