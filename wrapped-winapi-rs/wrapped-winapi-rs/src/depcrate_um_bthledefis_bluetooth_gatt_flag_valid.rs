// Generated macro for IS_BLUETOOTH_GATT_FLAG_VALID (function)
macro_rules! Depcrate_um_bthledefIS_BLUETOOTH_GATT_FLAG_VALID {
() => {
// Module: crate::um::bthledef
// Provides: {"IS_BLUETOOTH_GATT_FLAG_VALID"}
// Dependencies: {}
# [inline] pub fn IS_BLUETOOTH_GATT_FLAG_VALID (f : ULONG) -> bool { (f & ! BLUETOOTH_GATT_FLAG_VALID_MASK) == 0 }
};
}
