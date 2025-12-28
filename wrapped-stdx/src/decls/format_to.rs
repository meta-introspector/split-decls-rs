macro_rules! format_to {
    () => {
        # [doc = " Appends formatted string to a `String`."] # [macro_export] macro_rules ! format_to { ($ buf : expr) => () ; ($ buf : expr , $ lit : literal $ ($ arg : tt) *) => { { use :: std :: fmt :: Write as _ ; _ = $ buf . write_fmt (format_args ! ($ lit $ ($ arg) *)) } } ; }
    };
}

format_to!()