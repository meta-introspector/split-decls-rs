// Generated macro for split_unit_term (function)
macro_rules! Depcrate_units_helperssplit_unit_term {
() => {
// Module: crate::units::helpers
// Provides: {"split_unit_term"}
// Dependencies: {}
# [doc = " Splits a constant string into a tuple of (numerator, denominator)."] # [doc = " The numerator and denominator are represented as arrays of strings."] # [doc = " Examples:"] # [doc = " - \"1/2\" is split into ([\"1\"], [\"2\"])"] # [doc = " - \"1 * 2 / 3 * ft_to_m\" is split into ([\"1\", \"2\"], [\"3\" , \"ft_to_m\"])"] # [doc = " - \"/2\" is split into ([\"1\"], [\"2\"])"] # [doc = " - \"2\" is split into ([\"2\"], [\"1\"])"] # [doc = " - \"2/\" is split into ([\"2\"], [\"1\"])"] # [doc = " - \"1E2\" is split into ([\"1E2\"], [\"1\"])"] # [doc = " - \"1 2 * 3\" is an invalid constant string"] pub (crate) fn split_unit_term (constant_string : & str ,) -> Result < (Vec < String > , Vec < String >) , DataError > { let split : Vec < & str > = constant_string . split ('/') . collect () ; if split . len () > 2 { return Err (DataError :: custom ("Invalid constant string")) ; } let process_string = | s : & str | -> Vec < String > { if s . is_empty () { vec ! ["1" . to_string ()] } else { s . split ('*') . map (| s | s . trim () . to_string ()) . collect () } } ; let numerator_values = process_string (split . first () . unwrap_or (& "1")) ; let denominator_values = process_string (split . get (1) . unwrap_or (& "1")) ; if numerator_values . iter () . any (| s | s . chars () . any (char :: is_whitespace)) || denominator_values . iter () . any (| s | s . chars () . any (char :: is_whitespace)) { return Err (DataError :: custom ("The constant string contains internal white spaces" ,)) ; } Ok ((numerator_values , denominator_values)) }
};
}
