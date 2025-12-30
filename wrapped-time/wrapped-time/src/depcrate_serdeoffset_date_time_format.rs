// Generated macro for OFFSET_DATE_TIME_FORMAT (const)
macro_rules! Depcrate_serdeOFFSET_DATE_TIME_FORMAT {
() => {
// Module: crate::serde
// Provides: {"OFFSET_DATE_TIME_FORMAT"}
// Dependencies: {}
# [doc = " The format used when serializing and deserializing a human-readable `OffsetDateTime`."] # [cfg (feature = "parsing")] const OFFSET_DATE_TIME_FORMAT : & [BorrowedFormatItem < '_ >] = & [BorrowedFormatItem :: Compound (DATE_FORMAT) , BorrowedFormatItem :: Literal (b" ") , BorrowedFormatItem :: Compound (TIME_FORMAT) , BorrowedFormatItem :: Literal (b" ") , BorrowedFormatItem :: Compound (UTC_OFFSET_FORMAT) ,] ;
};
}
