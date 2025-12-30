// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_timeimpl_1044 {
() => {
// Module: crate::time
// Provides: {"impl_1044"}
// Dependencies: {}
impl SmartDisplay for Time { type Metadata = TimeMetadata ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let (subsecond_value , subsecond_width) = match self . nanosecond () { nanos if nanos % 10 != 0 => (nanos , 9) , nanos if (nanos / 10) % 10 != 0 => (nanos / 10 , 8) , nanos if (nanos / 100) % 10 != 0 => (nanos / 100 , 7) , nanos if (nanos / 1_000) % 10 != 0 => (nanos / 1_000 , 6) , nanos if (nanos / 10_000) % 10 != 0 => (nanos / 10_000 , 5) , nanos if (nanos / 100_000) % 10 != 0 => (nanos / 100_000 , 4) , nanos if (nanos / 1_000_000) % 10 != 0 => (nanos / 1_000_000 , 3) , nanos if (nanos / 10_000_000) % 10 != 0 => (nanos / 10_000_000 , 2) , nanos => (nanos / 100_000_000 , 1) , } ; let formatted_width = smart_display :: padded_width_of ! (self . hour . get () , ":" , self . minute . get () => width (2) fill ('0') , ":" , self . second . get () => width (2) fill ('0') , "." ,) + subsecond_width ; Metadata :: new (formatted_width , self , TimeMetadata { subsecond_width : subsecond_width . truncate () , subsecond_value , } ,) } # [inline] fn fmt_with_metadata (& self , f : & mut fmt :: Formatter < '_ > , metadata : Metadata < Self > ,) -> fmt :: Result { let subsecond_width = metadata . subsecond_width . extend () ; let subsecond_value = metadata . subsecond_value ; f . pad_with_width (metadata . unpadded_width () , format_args ! ("{}:{:02}:{:02}.{subsecond_value:0subsecond_width$}" , self . hour , self . minute , self . second) ,) } }
};
}
