// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1073 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1073"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl UtcDateTime { # [doc = " Format the `UtcDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , Some (self . date ()) , Some (self . time ()) , Some (UtcOffset :: UTC) ,) } # [doc = " Format the `UtcDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::format_description;"] # [doc = " # use time_macros::utc_datetime;"] # [doc = " let format = format_description::parse("] # [doc = "     \"[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \\"] # [doc = "          sign:mandatory]:[offset_minute]:[offset_second]\","] # [doc = " )?;"] # [doc = " assert_eq!("] # [doc = "     utc_datetime!(2020-01-02 03:04:05).format(&format)?,"] # [doc = "     \"2020-01-02 03:04:05 +00:00:00\""] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (Some (self . date ()) , Some (self . time ()) , Some (UtcOffset :: UTC)) } }
};
}
