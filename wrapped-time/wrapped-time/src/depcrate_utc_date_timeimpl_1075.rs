// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_utc_date_timeimpl_1075 {
() => {
// Module: crate::utc_date_time
// Provides: {"impl_1075"}
// Dependencies: {}
impl SmartDisplay for UtcDateTime { type Metadata = () ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let width = smart_display :: padded_width_of ! (self . date () , " " , self . time () , " +00") ; Metadata :: new (width , self , ()) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{} {} +00" , self . date () , self . time ()) ,) } }
};
}
