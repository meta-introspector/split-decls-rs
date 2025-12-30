// Generated macro for DATE_FORMAT (const)
macro_rules! Depcrate_serdeDATE_FORMAT {
() => {
// Module: crate::serde
// Provides: {"DATE_FORMAT"}
// Dependencies: {}
# [doc = " The format used when serializing and deserializing a human-readable `Date`."] # [cfg (feature = "parsing")] const DATE_FORMAT : & [BorrowedFormatItem < '_ >] = & [BorrowedFormatItem :: Component (Component :: Year (modifier :: Year :: default ())) , BorrowedFormatItem :: Literal (b"-") , BorrowedFormatItem :: Component (Component :: Month (modifier :: Month :: default ())) , BorrowedFormatItem :: Literal (b"-") , BorrowedFormatItem :: Component (Component :: Day (modifier :: Day :: default ())) ,] ;
};
}
