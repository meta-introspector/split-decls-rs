// Generated macro for impl_637 (impl)
macro_rules! Depcrate_offset_date_timeimpl_637 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_637"}
// Dependencies: {}
impl SmartDisplay for OffsetDateTime { type Metadata = () ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let width = smart_display :: padded_width_of ! (self . date () , " " , self . time () , " " , self . offset ()) ; Metadata :: new (width , self , ()) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{} {} {}" , self . date () , self . time () , self . offset ()) ,) } }
};
}
