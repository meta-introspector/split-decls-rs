// Generated macro for process_factor (function)
macro_rules! Depcrate_units_helpersprocess_factor {
() => {
// Module: crate::units::helpers
// Provides: {"process_factor"}
// Dependencies: {}
# [doc = " Processes a factor in the form of a string and returns a ScientificNumber."] # [doc = " Examples:"] # [doc = "     \"1\" is converted to ScientificNumber { clean_num: [\"1\"], clean_den: [\"1\"], exactness: Exact }"] # [doc = "     \"3 * ft_to_m\" is converted to ScientificNumber { clean_num: [\"3\", \"ft_to_m\"], clean_den: [\"1\"], exactness: Exact }"] # [doc = " NOTE:"] # [doc = "    If one of the constants in the factor is approximate, the whole factor is approximate."] pub (crate) fn process_factor (factor : & str , cons_map : & BTreeMap < & str , ScientificNumber > ,) -> Result < ScientificNumber , DataError > { let mut factor_parts = factor . split ('/') ; let factor_num_str = factor_parts . next () . unwrap_or ("0") . trim () ; let factor_den_str = factor_parts . next () . unwrap_or ("1") . trim () ; if factor_parts . next () . is_some () { return Err (DataError :: custom ("the factor is not a valid scientific notation number" ,)) ; } let mut result = process_factor_part (factor_num_str , cons_map) ? ; let factor_den_scientific = process_factor_part (factor_den_str , cons_map) ? ; result . clean_num . extend (factor_den_scientific . clean_den) ; result . clean_den . extend (factor_den_scientific . clean_num) ; if factor_den_scientific . exactness == Exactness :: Approximate { result . exactness = Exactness :: Approximate ; } Ok (result) }
};
}
