// Generated macro for UTC_OFFSET_FORMAT (const)
macro_rules! Depcrate_serdeUTC_OFFSET_FORMAT {
() => {
// Module: crate::serde
// Provides: {"UTC_OFFSET_FORMAT"}
// Dependencies: {}
# [doc = " The format used when serializing and deserializing a human-readable `UtcOffset`."] # [cfg (feature = "parsing")] const UTC_OFFSET_FORMAT : & [BorrowedFormatItem < '_ >] = & [BorrowedFormatItem :: Component (Component :: OffsetHour (const { let mut m = modifier :: OffsetHour :: default () ; m . sign_is_mandatory = true ; m } ,)) , BorrowedFormatItem :: Optional (& BorrowedFormatItem :: Compound (& [BorrowedFormatItem :: Literal (b":") , BorrowedFormatItem :: Component (Component :: OffsetMinute (const { modifier :: OffsetMinute :: default () } ,)) , BorrowedFormatItem :: Optional (& BorrowedFormatItem :: Compound (& [BorrowedFormatItem :: Literal (b":") , BorrowedFormatItem :: Component (Component :: OffsetSecond (const { modifier :: OffsetSecond :: default () } ,)) ,])) ,])) ,] ;
};
}
