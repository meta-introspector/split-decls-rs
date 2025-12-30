// Generated macro for parse_month (function)
macro_rules! Depcrate_parsing_componentparse_month {
() => {
// Module: crate::parsing::component
// Provides: {"parse_month"}
// Dependencies: {}
# [doc = " Parse the \"month\" component of a `Date`."] pub (crate) fn parse_month (input : & [u8] , modifiers : modifier :: Month ,) -> Option < ParsedItem < '_ , Month > > { use Month :: * ; let ParsedItem (remaining , value) = first_match (match modifiers . repr { modifier :: MonthRepr :: Numerical => { return exactly_n_digits_padded :: < 2 , _ > (modifiers . padding) (input) ? . flat_map (| n | Month :: from_number (n) . ok ()) ; } modifier :: MonthRepr :: Long => [(b"January" . as_slice () , January) , (b"February" . as_slice () , February) , (b"March" . as_slice () , March) , (b"April" . as_slice () , April) , (b"May" . as_slice () , May) , (b"June" . as_slice () , June) , (b"July" . as_slice () , July) , (b"August" . as_slice () , August) , (b"September" . as_slice () , September) , (b"October" . as_slice () , October) , (b"November" . as_slice () , November) , (b"December" . as_slice () , December) ,] , modifier :: MonthRepr :: Short => [(b"Jan" . as_slice () , January) , (b"Feb" . as_slice () , February) , (b"Mar" . as_slice () , March) , (b"Apr" . as_slice () , April) , (b"May" . as_slice () , May) , (b"Jun" . as_slice () , June) , (b"Jul" . as_slice () , July) , (b"Aug" . as_slice () , August) , (b"Sep" . as_slice () , September) , (b"Oct" . as_slice () , October) , (b"Nov" . as_slice () , November) , (b"Dec" . as_slice () , December) ,] , } , modifiers . case_sensitive ,) (input) ? ; Some (ParsedItem (remaining , value)) }
};
}
