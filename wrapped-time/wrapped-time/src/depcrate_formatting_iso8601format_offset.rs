// Generated macro for format_offset (function)
macro_rules! Depcrate_formatting_iso8601format_offset {
() => {
// Module: crate::formatting::iso8601
// Provides: {"format_offset"}
// Dependencies: {}
# [doc = " Format the UTC offset portion of ISO 8601."] # [inline] pub (super) fn format_offset < const CONFIG : EncodedConfig > (output : & mut (impl io :: Write + ? Sized) , offset : UtcOffset ,) -> Result < usize , error :: Format > { if Iso8601 :: < CONFIG > :: FORMAT_TIME && offset . is_utc () { return Ok (write (output , b"Z") ?) ; } let mut bytes = 0 ; let (hours , minutes , seconds) = offset . as_hms () ; if seconds != 0 { return Err (error :: Format :: InvalidComponent ("offset_second")) ; } bytes += write_if_else (output , offset . is_negative () , b"-" , b"+") ? ; bytes += format_number_pad_zero :: < 2 > (output , hours . unsigned_abs ()) ? ; if Iso8601 :: < CONFIG > :: OFFSET_PRECISION == OffsetPrecision :: Hour && minutes != 0 { return Err (error :: Format :: InvalidComponent ("offset_minute")) ; } else if Iso8601 :: < CONFIG > :: OFFSET_PRECISION == OffsetPrecision :: Minute { bytes += write_if (output , Iso8601 :: < CONFIG > :: USE_SEPARATORS , b":") ? ; bytes += format_number_pad_zero :: < 2 > (output , minutes . unsigned_abs ()) ? ; } Ok (bytes) }
};
}
