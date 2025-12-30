// Generated macro for impl_438 (impl)
macro_rules! Depcrate_formatting_formattableimpl_438 {
() => {
// Module: crate::formatting::formattable
// Provides: {"impl_438"}
// Dependencies: {}
impl < const CONFIG : EncodedConfig > sealed :: Sealed for Iso8601 < CONFIG > { # [inline] fn format_into (& self , output : & mut (impl io :: Write + ? Sized) , date : Option < Date > , time : Option < Time > , offset : Option < UtcOffset > ,) -> Result < usize , error :: Format > { let mut bytes = 0 ; if Self :: FORMAT_DATE { let date = date . ok_or (error :: Format :: InsufficientTypeInformation) ? ; bytes += iso8601 :: format_date :: < CONFIG > (output , date) ? ; } if Self :: FORMAT_TIME { let time = time . ok_or (error :: Format :: InsufficientTypeInformation) ? ; bytes += iso8601 :: format_time :: < CONFIG > (output , time) ? ; } if Self :: FORMAT_OFFSET { let offset = offset . ok_or (error :: Format :: InsufficientTypeInformation) ? ; bytes += iso8601 :: format_offset :: < CONFIG > (output , offset) ? ; } if bytes == 0 { panic ! ("attempted to format a parsing-only format description") ; } Ok (bytes) } }
};
}
