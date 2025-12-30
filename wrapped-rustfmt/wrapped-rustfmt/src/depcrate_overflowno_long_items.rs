// Generated macro for no_long_items (function)
macro_rules! Depcrate_overflowno_long_items {
() => {
// Module: crate::overflow
// Provides: {"no_long_items"}
// Dependencies: {}
fn no_long_items (list : & [ListItem] , short_array_element_width_threshold : usize) -> bool { list . iter () . all (| item | item . inner_as_ref () . len () <= short_array_element_width_threshold) }
};
}
