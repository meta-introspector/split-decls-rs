// Generated macro for fmt_offset_hour (function)
macro_rules! Depcrate_formattingfmt_offset_hour {
() => {
// Module: crate::formatting
// Provides: {"fmt_offset_hour"}
// Dependencies: {}
# [doc = " Format the offset hour into the designated output."] # [inline] fn fmt_offset_hour (output : & mut (impl io :: Write + ? Sized) , offset : UtcOffset , modifier :: OffsetHour { padding , sign_is_mandatory , } : modifier :: OffsetHour ,) -> Result < usize , io :: Error > { let mut bytes = 0 ; if offset . is_negative () { bytes += write (output , b"-") ? ; } else if sign_is_mandatory { bytes += write (output , b"+") ? ; } bytes += format_number :: < 2 > (output , offset . whole_hours () . unsigned_abs () , padding) ? ; Ok (bytes) }
};
}
