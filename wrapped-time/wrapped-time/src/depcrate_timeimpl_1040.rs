// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_timeimpl_1040 {
() => {
// Module: crate::time
// Provides: {"impl_1040"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl Time { # [doc = " Format the `Time` using the provided [format description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , None , Some (self) , None) } # [doc = " Format the `Time` using the provided [format description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::format_description;"] # [doc = " # use time_macros::time;"] # [doc = " let format = format_description::parse(\"[hour]:[minute]:[second]\")?;"] # [doc = " assert_eq!(time!(12:00).format(&format)?, \"12:00:00\");"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (None , Some (self) , None) } }
};
}
