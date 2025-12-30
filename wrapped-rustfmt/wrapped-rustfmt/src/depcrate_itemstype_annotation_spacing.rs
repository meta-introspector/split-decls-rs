// Generated macro for type_annotation_spacing (function)
macro_rules! Depcrate_itemstype_annotation_spacing {
() => {
// Module: crate::items
// Provides: {"type_annotation_spacing"}
// Dependencies: {}
fn type_annotation_spacing (config : & Config) -> (& str , & str) { (if config . space_before_colon () { " " } else { "" } , if config . space_after_colon () { " " } else { "" } ,) }
};
}
