// Generated macro for TIME_FORMAT (const)
macro_rules! Depcrate_serdeTIME_FORMAT {
() => {
// Module: crate::serde
// Provides: {"TIME_FORMAT"}
// Dependencies: {}
# [doc = " The format used when serializing and deserializing a human-readable `Time`."] # [cfg (feature = "parsing")] const TIME_FORMAT : & [BorrowedFormatItem < '_ >] = & [BorrowedFormatItem :: Component (Component :: Hour (modifier :: Hour :: default ())) , BorrowedFormatItem :: Literal (b":") , BorrowedFormatItem :: Component (Component :: Minute (modifier :: Minute :: default ())) , BorrowedFormatItem :: Literal (b":") , BorrowedFormatItem :: Component (Component :: Second (modifier :: Second :: default ())) , BorrowedFormatItem :: Literal (b".") , BorrowedFormatItem :: Component (Component :: Subsecond (modifier :: Subsecond :: default ())) ,] ;
};
}
