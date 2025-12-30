// Generated macro for impl_948 (impl)
macro_rules! Depcrate_serde_visitorimpl_948 {
() => {
// Module: crate::serde::visitor
// Provides: {"impl_948"}
// Dependencies: {}
impl de :: Visitor < '_ > for Visitor < Month > { type Value = Month ; # [inline] fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a `Month`") } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Month , E > { match value { "January" => Ok (Month :: January) , "February" => Ok (Month :: February) , "March" => Ok (Month :: March) , "April" => Ok (Month :: April) , "May" => Ok (Month :: May) , "June" => Ok (Month :: June) , "July" => Ok (Month :: July) , "August" => Ok (Month :: August) , "September" => Ok (Month :: September) , "October" => Ok (Month :: October) , "November" => Ok (Month :: November) , "December" => Ok (Month :: December) , _ => Err (E :: invalid_value (de :: Unexpected :: Str (value) , & "a `Month`")) , } } # [inline] fn visit_u64 < E : de :: Error > (self , value : u64) -> Result < Month , E > { match value { 1 => Ok (Month :: January) , 2 => Ok (Month :: February) , 3 => Ok (Month :: March) , 4 => Ok (Month :: April) , 5 => Ok (Month :: May) , 6 => Ok (Month :: June) , 7 => Ok (Month :: July) , 8 => Ok (Month :: August) , 9 => Ok (Month :: September) , 10 => Ok (Month :: October) , 11 => Ok (Month :: November) , 12 => Ok (Month :: December) , _ => Err (E :: invalid_value (de :: Unexpected :: Unsigned (value) , & "a value in the range 1..=12" ,)) , } } }
};
}
