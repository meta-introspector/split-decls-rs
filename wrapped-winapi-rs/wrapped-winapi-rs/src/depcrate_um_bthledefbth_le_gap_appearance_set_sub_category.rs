// Generated macro for BTH_LE_GAP_APPEARANCE_SET_SUB_CATEGORY (function)
macro_rules! Depcrate_um_bthledefBTH_LE_GAP_APPEARANCE_SET_SUB_CATEGORY {
() => {
// Module: crate::um::bthledef
// Provides: {"BTH_LE_GAP_APPEARANCE_SET_SUB_CATEGORY"}
// Dependencies: {}
# [inline] pub fn BTH_LE_GAP_APPEARANCE_SET_SUB_CATEGORY (a : & mut USHORT , s : UCHAR) { * a = (* a & ! BTH_LE_GAP_APPEARANCE_SUB_CATEGORY_MASK) | (s as u16) ; }
};
}
