// Generated macro for BTH_LE_GAP_APPEARANCE_SET_CATEGORY (function)
macro_rules! Depcrate_um_bthledefBTH_LE_GAP_APPEARANCE_SET_CATEGORY {
() => {
// Module: crate::um::bthledef
// Provides: {"BTH_LE_GAP_APPEARANCE_SET_CATEGORY"}
// Dependencies: {}
# [inline] pub fn BTH_LE_GAP_APPEARANCE_SET_CATEGORY (a : & mut USHORT , c : USHORT) { * a = (* a & ! BTH_LE_GAP_APPEARANCE_CATEGORY_MASK) | (c << BTH_LE_GAP_APPEARANCE_CATEGORY_OFFSET) ; }
};
}
