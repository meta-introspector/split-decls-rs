// Generated macro for print_property_values (function)
macro_rules! Depcrate_utilprint_property_values {
() => {
// Module: crate::util
// Provides: {"print_property_values"}
// Dependencies: {}
# [doc = " Print the property values (and their aliases) for the given property."] pub fn print_property_values (propvals : & PropertyValues , property : & str ,) -> Result < () > { let by_alias = propvals . values (property) ? ; let mut by_canonical : BTreeMap < & str , Vec < & str > > = BTreeMap :: new () ; for (alias , canonical) in by_alias { by_canonical . entry (& * * canonical) . or_insert (vec ! []) . push (& * * alias) ; } for (canonical , mut aliases) in by_canonical { aliases . sort () ; println ! ("{} (aliases: {})" , canonical , aliases . join (", ")) ; } Ok (()) }
};
}
