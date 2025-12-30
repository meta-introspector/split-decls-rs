// Generated macro for fmt_unix_timestamp (function)
macro_rules! Depcrate_formattingfmt_unix_timestamp {
() => {
// Module: crate::formatting
// Provides: {"fmt_unix_timestamp"}
// Dependencies: {}
# [doc = " Format the Unix timestamp into the designated output."] # [inline] fn fmt_unix_timestamp (output : & mut (impl io :: Write + ? Sized) , date : Date , time : Time , offset : UtcOffset , modifier :: UnixTimestamp { precision , sign_is_mandatory , } : modifier :: UnixTimestamp ,) -> Result < usize , io :: Error > { let date_time = OffsetDateTime :: new_in_offset (date , time , offset) . to_offset (UtcOffset :: UTC) ; if date_time < OffsetDateTime :: UNIX_EPOCH { write (output , b"-") ? ; } else if sign_is_mandatory { write (output , b"+") ? ; } match precision { modifier :: UnixTimestampPrecision :: Second => { format_number_pad_none (output , date_time . unix_timestamp () . unsigned_abs ()) } modifier :: UnixTimestampPrecision :: Millisecond => format_number_pad_none (output , (date_time . unix_timestamp_nanos () / Nanosecond :: per_t :: < i128 > (Millisecond)) . unsigned_abs () ,) , modifier :: UnixTimestampPrecision :: Microsecond => format_number_pad_none (output , (date_time . unix_timestamp_nanos () / Nanosecond :: per_t :: < i128 > (Microsecond)) . unsigned_abs () ,) , modifier :: UnixTimestampPrecision :: Nanosecond => { format_number_pad_none (output , date_time . unix_timestamp_nanos () . unsigned_abs ()) } } }
};
}
