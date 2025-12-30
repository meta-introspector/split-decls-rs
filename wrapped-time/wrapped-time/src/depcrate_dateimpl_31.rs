// Generated macro for impl_31 (impl)
macro_rules! Depcrate_dateimpl_31 {
() => {
// Module: crate::date
// Provides: {"impl_31"}
// Dependencies: {}
impl SmartDisplay for Date { type Metadata = DateMetadata ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let (year , month , day) = self . to_calendar_date () ; let mut year_width = cmp :: max (year . unsigned_abs () . num_digits () , 4) ; let display_sign = if ! (0 .. 10_000) . contains (& year) { year_width += 1 ; true } else { false } ; let formatted_width = year_width . extend :: < usize > () + smart_display :: padded_width_of ! ("-" , u8 :: from (month) => width (2) , "-" , day => width (2) ,) ; Metadata :: new (formatted_width , self , DateMetadata { year_width , display_sign , year , month : u8 :: from (month) , day , } ,) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { let DateMetadata { year_width , display_sign , year , month , day , } = * metadata ; let year_width = year_width . extend () ; if display_sign { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{year:+0year_width$}-{month:02}-{day:02}") ,) } else { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{year:0year_width$}-{month:02}-{day:02}") ,) } } }
};
}
