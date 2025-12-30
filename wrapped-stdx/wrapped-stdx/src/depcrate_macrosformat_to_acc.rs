// Generated macro for format_to_acc (macro)
macro_rules! Depcrate_macrosformat_to_acc {
() => {
// Module: crate::macros
// Provides: {"format_to_acc"}
// Dependencies: {}
# [doc = " Appends formatted string to a `String` and returns the `String`."] # [doc = ""] # [doc = " Useful for folding iterators into a `String`."] # [macro_export] macro_rules ! format_to_acc { ($ buf : expr , $ lit : literal $ ($ arg : tt) *) => { { use :: std :: fmt :: Write as _ ; _ = $ buf . write_fmt (format_args ! ($ lit $ ($ arg) *)) ; $ buf } } ; }
};
}
