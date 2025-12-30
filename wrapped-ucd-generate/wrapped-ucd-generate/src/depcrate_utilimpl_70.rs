// Generated macro for impl_70 (impl)
macro_rules! Depcrate_utilimpl_70 {
() => {
// Module: crate::util
// Provides: {"impl_70"}
// Dependencies: {}
impl Filter { # [doc = " Create a new include/exclude filter from the given lists."] # [doc = ""] # [doc = " Each list should be a comma separated list of property names (or"] # [doc = " values), and they may be aliases. The canonicalization function given"] # [doc = " should convert all such values into their canonical form."] pub fn new < F : FnMut (& str) -> Result < String > > (include_list : Option < String > , exclude_list : Option < String > , mut canonicalize : F ,) -> Result < Filter > { let (mut include , mut exclude) = (BTreeSet :: new () , BTreeSet :: new ()) ; if let Some (include_list) = include_list { for name in include_list . split (",") { include . insert (canonicalize (name . trim ()) ? . to_string ()) ; } } if let Some (exclude_list) = exclude_list { for name in exclude_list . split (",") { exclude . insert (canonicalize (name . trim ()) ? . to_string ()) ; } } Ok (Filter { include , exclude }) } # [doc = " Whether the given name passes this filter or not."] pub fn contains (& self , name : & str) -> bool { if self . exclude . contains (name) { false } else { self . include . is_empty () || self . include . contains (name) } } }
};
}
