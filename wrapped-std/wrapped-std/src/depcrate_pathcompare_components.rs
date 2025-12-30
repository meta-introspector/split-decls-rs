// Generated macro for compare_components (function)
macro_rules! Depcrate_pathcompare_components {
() => {
// Module: crate::path
// Provides: {"compare_components"}
// Dependencies: {}
fn compare_components (mut left : Components < '_ > , mut right : Components < '_ >) -> cmp :: Ordering { if left . prefix . is_none () && right . prefix . is_none () && left . front == right . front { let first_difference = match left . path . iter () . zip (right . path) . position (| (& a , & b) | a != b) { None if left . path . len () == right . path . len () => return cmp :: Ordering :: Equal , None => left . path . len () . min (right . path . len ()) , Some (diff) => diff , } ; if let Some (previous_sep) = left . path [.. first_difference] . iter () . rposition (| & b | left . is_sep_byte (b)) { let mismatched_component_start = previous_sep + 1 ; left . path = & left . path [mismatched_component_start ..] ; left . front = State :: Body ; right . path = & right . path [mismatched_component_start ..] ; right . front = State :: Body ; } } Iterator :: cmp (left , right) }
};
}
