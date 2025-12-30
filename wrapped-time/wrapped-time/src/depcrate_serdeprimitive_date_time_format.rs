// Generated macro for PRIMITIVE_DATE_TIME_FORMAT (const)
macro_rules! Depcrate_serdePRIMITIVE_DATE_TIME_FORMAT {
() => {
// Module: crate::serde
// Provides: {"PRIMITIVE_DATE_TIME_FORMAT"}
// Dependencies: {}
# [doc = " The format used when serializing and deserializing a human-readable `PrimitiveDateTime`."] # [cfg (feature = "parsing")] const PRIMITIVE_DATE_TIME_FORMAT : & [BorrowedFormatItem < '_ >] = & [BorrowedFormatItem :: Compound (DATE_FORMAT) , BorrowedFormatItem :: Literal (b" ") , BorrowedFormatItem :: Compound (TIME_FORMAT) ,] ;
};
}
