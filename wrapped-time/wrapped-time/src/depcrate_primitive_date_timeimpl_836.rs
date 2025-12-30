// Generated macro for impl_836 (impl)
macro_rules! Depcrate_primitive_date_timeimpl_836 {
() => {
// Module: crate::primitive_date_time
// Provides: {"impl_836"}
// Dependencies: {}
impl SmartDisplay for PrimitiveDateTime { type Metadata = () ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let width = smart_display :: padded_width_of ! (self . date , " " , self . time) ; Metadata :: new (width , self , ()) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { f . pad_with_width (metadata . unpadded_width () , format_args ! ("{} {}" , self . date , self . time) ,) } }
};
}
