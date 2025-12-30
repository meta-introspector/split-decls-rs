// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_utc_offsetimpl_1114 {
() => {
// Module: crate::utc_offset
// Provides: {"impl_1114"}
// Dependencies: {}
# [cfg (feature = "formatting")] impl UtcOffset { # [doc = " Format the `UtcOffset` using the provided [format description](crate::format_description)."] # [inline] pub fn format_into (self , output : & mut (impl io :: Write + ? Sized) , format : & (impl Formattable + ? Sized) ,) -> Result < usize , error :: Format > { format . format_into (output , None , None , Some (self)) } # [doc = " Format the `UtcOffset` using the provided [format description](crate::format_description)."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use time::format_description;"] # [doc = " # use time_macros::offset;"] # [doc = " let format = format_description::parse(\"[offset_hour sign:mandatory]:[offset_minute]\")?;"] # [doc = " assert_eq!(offset!(+1).format(&format)?, \"+01:00\");"] # [doc = " # Ok::<_, time::Error>(())"] # [doc = " ```"] # [inline] pub fn format (self , format : & (impl Formattable + ? Sized)) -> Result < String , error :: Format > { format . format (None , None , Some (self)) } }
};
}
