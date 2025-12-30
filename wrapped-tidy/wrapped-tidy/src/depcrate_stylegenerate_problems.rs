// Generated macro for generate_problems (function)
macro_rules! Depcrate_stylegenerate_problems {
() => {
// Module: crate::style
// Provides: {"generate_problems"}
// Dependencies: {}
fn generate_problems < 'a > (consts : & 'a [u32] , letter_digit : & 'a FxHashMap < char , char > ,) -> impl Iterator < Item = u32 > + 'a { consts . iter () . flat_map (move | const_value | { let problem = letter_digit . iter () . fold (format ! ("{const_value:X}") , | acc , (key , value) | { acc . replace (& value . to_string () , & key . to_string ()) }) ; let indexes : Vec < usize > = problem . chars () . enumerate () . filter_map (| (index , c) | if letter_digit . contains_key (& c) { Some (index) } else { None }) . collect () ; (0 .. 1 << indexes . len ()) . map (move | i | { u32 :: from_str_radix (& problem . chars () . enumerate () . map (| (index , c) | { if let Some (pos) = indexes . iter () . position (| & x | x == index) { if (i >> pos) & 1 == 1 { letter_digit [& c] } else { c } } else { c } }) . collect :: < String > () , 0x10 ,) . unwrap () }) }) }
};
}
