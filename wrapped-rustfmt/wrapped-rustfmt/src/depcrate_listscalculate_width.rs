// Generated macro for calculate_width (function)
macro_rules! Depcrate_listscalculate_width {
() => {
// Module: crate::lists
// Provides: {"calculate_width"}
// Dependencies: {}
# [doc = " Returns the count and total width of the list items."] fn calculate_width < I , T > (items : I) -> (usize , usize) where I : IntoIterator < Item = T > , T : AsRef < ListItem > , { items . into_iter () . map (| item | total_item_width (item . as_ref ())) . fold ((0 , 0) , | acc , l | (acc . 0 + 1 , acc . 1 + l)) }
};
}
