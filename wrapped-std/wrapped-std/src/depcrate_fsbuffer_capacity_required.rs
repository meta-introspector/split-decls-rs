// Generated macro for buffer_capacity_required (function)
macro_rules! Depcrate_fsbuffer_capacity_required {
() => {
// Module: crate::fs
// Provides: {"buffer_capacity_required"}
// Dependencies: {}
# [doc = " Indicates how much extra capacity is needed to read the rest of the file."] fn buffer_capacity_required (mut file : & File) -> Option < usize > { let size = file . metadata () . map (| m | m . len ()) . ok () ? ; let pos = file . stream_position () . ok () ? ; Some (size . saturating_sub (pos) as usize) }
};
}
