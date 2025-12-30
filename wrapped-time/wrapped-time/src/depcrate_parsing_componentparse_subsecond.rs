// Generated macro for parse_subsecond (function)
macro_rules! Depcrate_parsing_componentparse_subsecond {
() => {
// Module: crate::parsing::component
// Provides: {"parse_subsecond"}
// Dependencies: {}
# [doc = " Parse the \"subsecond\" component of a `Time`."] pub (crate) fn parse_subsecond (input : & [u8] , modifiers : modifier :: Subsecond ,) -> Option < ParsedItem < '_ , u32 > > { use modifier :: SubsecondDigits :: * ; Some (match modifiers . digits { One => exactly_n_digits :: < 1 , u32 > (input) ? . map (| v | v * 100_000_000) , Two => exactly_n_digits :: < 2 , u32 > (input) ? . map (| v | v * 10_000_000) , Three => exactly_n_digits :: < 3 , u32 > (input) ? . map (| v | v * 1_000_000) , Four => exactly_n_digits :: < 4 , u32 > (input) ? . map (| v | v * 100_000) , Five => exactly_n_digits :: < 5 , u32 > (input) ? . map (| v | v * 10_000) , Six => exactly_n_digits :: < 6 , u32 > (input) ? . map (| v | v * 1_000) , Seven => exactly_n_digits :: < 7 , u32 > (input) ? . map (| v | v * 100) , Eight => exactly_n_digits :: < 8 , u32 > (input) ? . map (| v | v * 10) , Nine => exactly_n_digits :: < 9 , _ > (input) ? , OneOrMore => { let ParsedItem (mut input , mut value) = any_digit (input) ? . map (| v | (v - b'0') . extend :: < u32 > () * 100_000_000) ; let mut multiplier = 10_000_000 ; while let Some (ParsedItem (new_input , digit)) = any_digit (input) { value += (digit - b'0') . extend :: < u32 > () * multiplier ; input = new_input ; multiplier /= 10 ; } ParsedItem (input , value) } }) }
};
}
