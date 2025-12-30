// Generated macro for impl_603 (impl)
macro_rules! Depcrate_monthimpl_603 {
() => {
// Module: crate::month
// Provides: {"impl_603"}
// Dependencies: {}
impl SmartDisplay for Month { type Metadata = MonthMetadata ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { match self { January => Metadata :: new (7 , self , MonthMetadata) , February => Metadata :: new (8 , self , MonthMetadata) , March => Metadata :: new (5 , self , MonthMetadata) , April => Metadata :: new (5 , self , MonthMetadata) , May => Metadata :: new (3 , self , MonthMetadata) , June => Metadata :: new (4 , self , MonthMetadata) , July => Metadata :: new (4 , self , MonthMetadata) , August => Metadata :: new (6 , self , MonthMetadata) , September => Metadata :: new (9 , self , MonthMetadata) , October => Metadata :: new (7 , self , MonthMetadata) , November => Metadata :: new (8 , self , MonthMetadata) , December => Metadata :: new (8 , self , MonthMetadata) , } } # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (match self { January => "January" , February => "February" , March => "March" , April => "April" , May => "May" , June => "June" , July => "July" , August => "August" , September => "September" , October => "October" , November => "November" , December => "December" , }) } }
};
}
