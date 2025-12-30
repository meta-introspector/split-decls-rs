// Generated macro for impl_1141 (impl)
macro_rules! Depcrate_weekdayimpl_1141 {
() => {
// Module: crate::weekday
// Provides: {"impl_1141"}
// Dependencies: {}
impl SmartDisplay for Weekday { type Metadata = WeekdayMetadata ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { match self { Monday => Metadata :: new (6 , self , WeekdayMetadata) , Tuesday => Metadata :: new (7 , self , WeekdayMetadata) , Wednesday => Metadata :: new (9 , self , WeekdayMetadata) , Thursday => Metadata :: new (8 , self , WeekdayMetadata) , Friday => Metadata :: new (6 , self , WeekdayMetadata) , Saturday => Metadata :: new (8 , self , WeekdayMetadata) , Sunday => Metadata :: new (6 , self , WeekdayMetadata) , } } # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (match self { Monday => "Monday" , Tuesday => "Tuesday" , Wednesday => "Wednesday" , Thursday => "Thursday" , Friday => "Friday" , Saturday => "Saturday" , Sunday => "Sunday" , }) } }
};
}
