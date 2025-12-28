macro_rules! format_to_acc {
    () => {
        # [doc = " Appends formatted string to a `String` and returns the `String`."] # [doc = ""] # [doc = " Useful for folding iterators into a `String`."] # [macro_export] macro_rules ! format_to_acc { ($ buf : expr , $ lit : literal $ ($ arg : tt) *) => { { use :: std :: fmt :: Write as _ ; _ = $ buf . write_fmt (format_args ! ($ lit $ ($ arg) *)) ; $ buf } } ; }
    };
}

format_to_acc!();