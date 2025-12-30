// Generated macro for impl_635 (impl)
macro_rules! Depcrate_offset_date_timeimpl_635 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_635"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl OffsetDateTime { # [doc = " Format the `OffsetDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , Some (self . date ()) , Some (self . time ()) , Some (self . offset ()) ,) } # [doc = " Format the `OffsetDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::format_description;"] # [doc = " # use time_macros::datetime;"] # [doc = " let format = format_description::parse("] # [doc = "     \"[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \\"] # [doc = "          sign:mandatory]:[offset_minute]:[offset_second]\","] # [doc = " )?;"] # [doc = " assert_eq!("] # [doc = "     datetime!(2020-01-02 03:04:05 +06:07:08).format(&format)?,"] # [doc = "     \"2020-01-02 03:04:05 +06:07:08\""] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (Some (self . date ()) , Some (self . time ()) , Some (self . offset ())) } }
};
}
