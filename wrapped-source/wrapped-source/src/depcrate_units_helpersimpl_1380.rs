// Generated macro for impl_1380 (impl)
macro_rules! Depcrate_units_helpersimpl_1380 {
() => {
// Module: crate::units::helpers
// Provides: {"impl_1380"}
// Dependencies: {}
impl GeneralNonScientificNumber { fn new (num : & [String] , den : & [String] , exactness : Exactness) -> Self { let mut constant = GeneralNonScientificNumber { clean_num : Vec :: new () , clean_den : Vec :: new () , non_scientific_num : VecDeque :: new () , non_scientific_den : VecDeque :: new () , exactness , } ; for n in num { if is_scientific_number (n) { constant . clean_num . push (n . clone ()) ; } else { constant . non_scientific_num . push_back (n . clone ()) ; } } for d in den { if is_scientific_number (d) { constant . clean_den . push (d . clone ()) ; } else { constant . non_scientific_den . push_back (d . clone ()) ; } } constant } # [doc = " Determines if the constant is free of any non_scientific elements."] fn is_free_of_non_scientific (& self) -> bool { self . non_scientific_num . is_empty () && self . non_scientific_den . is_empty () } }
};
}
