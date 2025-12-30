// Generated macro for IsBthLEUuidMatch (function)
macro_rules! Depcrate_um_bthledefIsBthLEUuidMatch {
() => {
// Module: crate::um::bthledef
// Provides: {"IsBthLEUuidMatch"}
// Dependencies: {}
# [inline] pub fn IsBthLEUuidMatch (uuid1 : & BTH_LE_UUID , uuid2 : & BTH_LE_UUID) -> bool { fn is_bluetooth_le_uuid (uuid : & GUID) -> bool { uuid . Data2 == BTH_LE_ATT_BLUETOOTH_BASE_GUID . Data2 && uuid . Data3 == BTH_LE_ATT_BLUETOOTH_BASE_GUID . Data3 && uuid . Data4 == BTH_LE_ATT_BLUETOOTH_BASE_GUID . Data4 } unsafe { match (uuid1 . IsShortUuid != 0 , uuid2 . IsShortUuid != 0) { (true , true) => uuid1 . Value . ShortUuid () == uuid2 . Value . ShortUuid () , (false , false) => IsEqualGUID (uuid1 . Value . LongUuid () , uuid2 . Value . LongUuid ()) , (true , false) => is_bluetooth_le_uuid (uuid2 . Value . LongUuid ()) && ((* uuid1 . Value . ShortUuid ()) as u32) == uuid2 . Value . LongUuid () . Data1 , (false , true) => is_bluetooth_le_uuid (uuid1 . Value . LongUuid ()) && ((* uuid2 . Value . ShortUuid ()) as u32) == uuid1 . Value . LongUuid () . Data1 , } } }
};
}
