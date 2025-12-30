// Generated macro for unique (function)
macro_rules! Depcrate_algorithms_utilsunique {
() => {
// Module: crate::algorithms::utils
// Provides: {"unique"}
// Dependencies: {}
# [doc = " Returns only unique items in the sequence as vector."] # [doc = ""] # [doc = " Each item is wrapped in a [`UniqueItem`] so that both the value and the"] # [doc = " index can be extracted."] pub fn unique < Idx > (lookup : & Idx , range : Range < usize >) -> Vec < UniqueItem < '_ , Idx > > where Idx : Index < usize > + ? Sized , Idx :: Output : Hash + Eq , { let mut by_item = HashMap :: new () ; for index in range { match by_item . entry (& lookup [index]) { Entry :: Vacant (entry) => { entry . insert (Some (index)) ; } Entry :: Occupied (mut entry) => { let entry = entry . get_mut () ; if entry . is_some () { * entry = None } } } } let mut rv = by_item . into_iter () . filter_map (| (_ , x) | x) . map (| index | UniqueItem { lookup , index }) . collect :: < Vec < _ > > () ; rv . sort_by_key (| a | a . original_index ()) ; rv }
};
}
