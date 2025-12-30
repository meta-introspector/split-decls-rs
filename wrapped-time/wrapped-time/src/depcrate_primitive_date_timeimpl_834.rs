// Generated macro for impl_834 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_834 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_834"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl PrimitiveDateTime { # [doc = " Format the `PrimitiveDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , Some (self . date) , Some (self . time) , None) } # [doc = " Format the `PrimitiveDateTime` using the provided [format"] # [doc = " description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::format_description;"] # [doc = " # use time_macros::datetime;"] # [doc = " let format = format_description::parse(\"[year]-[month]-[day] [hour]:[minute]:[second]\")?;"] # [doc = " assert_eq!("] # [doc = "     datetime!(2020-01-02 03:04:05).format(&format)?,"] # [doc = "     \"2020-01-02 03:04:05\""] # [doc = " );"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (Some (self . date) , Some (self . time) , None) } }
};
}
