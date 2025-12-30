// Generated macro for process_factor_part (function)
macro_rules! Depcrate_units_helpersprocess_factor_part {
() => {
// Module: crate::units::helpers
// Provides: {"process_factor_part"}
// Dependencies: {}
pub (crate) fn process_factor_part (factor_part : & str , cons_map : & BTreeMap < & str , ScientificNumber > ,) -> Result < ScientificNumber , DataError > { if factor_part . contains ('/') { return Err (DataError :: custom ("the factor part is fractional number")) ; } let mut result = ScientificNumber { clean_num : Vec :: new () , clean_den : Vec :: new () , exactness : Exactness :: Exact , } ; let factor_parts = factor_part . split ('*') ; for factor in factor_parts { if let Some (cons) = cons_map . get (factor . trim ()) { result . clean_num . extend (cons . clean_num . clone ()) ; result . clean_den . extend (cons . clean_den . clone ()) ; if cons . exactness == Exactness :: Approximate { result . exactness = Exactness :: Approximate ; } } else { result . clean_num . push (factor . trim () . to_string ()) ; } } Ok (result) }
};
}
