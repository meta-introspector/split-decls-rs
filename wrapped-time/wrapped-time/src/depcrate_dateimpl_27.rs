// Generated macro for impl_27 (impl)
macro_rules! Depcrate_dateimpl_27 {
() => {
// Module: crate::date
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl Date { # [doc = " Format the `Date` using the provided [format description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , Some (self) , None , None) } # [doc = " Format the `Date` using the provided [format description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::{format_description};"] # [doc = " # use time_macros::date;"] # [doc = " let format = format_description::parse(\"[year]-[month]-[day]\")?;"] # [doc = " assert_eq!(date!(2020-01-02).format(&format)?, \"2020-01-02\");"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (Some (self) , None , None) } }
};
}
