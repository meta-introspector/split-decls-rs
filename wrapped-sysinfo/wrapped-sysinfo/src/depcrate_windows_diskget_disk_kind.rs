// Generated macro for get_disk_kind (function)
macro_rules! Depcrate_windows_diskget_disk_kind {
() => {
// Module: crate::windows::disk
// Provides: {"get_disk_kind"}
// Dependencies: {}
unsafe fn get_disk_kind (handle : & HandleWrapper) -> DiskKind { let spq_trim = STORAGE_PROPERTY_QUERY { PropertyId : StorageDeviceSeekPenaltyProperty , QueryType : PropertyStandardQuery , AdditionalParameters : [0] , } ; let mut result : DEVICE_SEEK_PENALTY_DESCRIPTOR = unsafe { std :: mem :: zeroed () } ; let mut dw_size = 0 ; let device_io_control = unsafe { DeviceIoControl (handle . 0 , IOCTL_STORAGE_QUERY_PROPERTY , Some (& spq_trim as * const STORAGE_PROPERTY_QUERY as * const _) , size_of :: < STORAGE_PROPERTY_QUERY > () as _ , Some (& mut result as * mut DEVICE_SEEK_PENALTY_DESCRIPTOR as * mut _) , size_of :: < DEVICE_SEEK_PENALTY_DESCRIPTOR > () as _ , Some (& mut dw_size) , None ,) . is_ok () } ; if ! device_io_control || dw_size != size_of :: < DEVICE_SEEK_PENALTY_DESCRIPTOR > () as u32 { DiskKind :: Unknown (- 1) } else { let is_hdd = result . IncursSeekPenalty ; if is_hdd { DiskKind :: HDD } else { DiskKind :: SSD } } }
};
}
