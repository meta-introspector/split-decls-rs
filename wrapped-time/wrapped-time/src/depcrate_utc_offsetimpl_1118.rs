// Generated macro for impl_1118 (impl)
macro_rules! Depcrate_utc_offsetimpl_1118 {
() => {
// Module: crate::utc_offset
// Provides: {"impl_1118"}
// Dependencies: {}
impl SmartDisplay for UtcOffset { type Metadata = UtcOffsetMetadata ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let sign = if self . is_negative () { '-' } else { '+' } ; let width = smart_display :: padded_width_of ! (sign , self . hours . abs () => width (2) , ":" , self . minutes . abs () => width (2) , ":" , self . seconds . abs () => width (2) ,) ; Metadata :: new (width , self , UtcOffsetMetadata) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{}{:02}:{:02}:{:02}" , if self . is_negative () { '-' } else { '+' } , self . hours . abs () , self . minutes . abs () , self . seconds . abs () ,) ,) } }
};
}
