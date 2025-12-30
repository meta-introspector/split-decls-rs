// Generated macro for impl_947 (impl)
macro_rules! Depcrate_serde_visitorimpl_947 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_947"}
// Dependencies: {}
impl de :: Visitor < '_ > for Visitor < Weekday > { type Value = Weekday ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `Weekday`") } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Weekday , E > { match value { "Monday" => Ok (Weekday :: Monday) , "Tuesday" => Ok (Weekday :: Tuesday) , "Wednesday" => Ok (Weekday :: Wednesday) , "Thursday" => Ok (Weekday :: Thursday) , "Friday" => Ok (Weekday :: Friday) , "Saturday" => Ok (Weekday :: Saturday) , "Sunday" => Ok (Weekday :: Sunday) , _ => Err (E :: invalid_value (de :: Unexpected :: Str (value) , & "a `Weekday`")) , } } # [inline] fn visit_u64 < E : de :: Error > (self , value : u64) -> Result < Weekday , E > { match value { 1 => Ok (Weekday :: Monday) , 2 => Ok (Weekday :: Tuesday) , 3 => Ok (Weekday :: Wednesday) , 4 => Ok (Weekday :: Thursday) , 5 => Ok (Weekday :: Friday) , 6 => Ok (Weekday :: Saturday) , 7 => Ok (Weekday :: Sunday) , _ => Err (E :: invalid_value (de :: Unexpected :: Unsigned (value) , & "a value in the range 1..=7" ,)) , } } }
};
}
