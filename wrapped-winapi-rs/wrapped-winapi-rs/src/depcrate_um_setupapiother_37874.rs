// Generated macro for other_37874 (other)
macro_rules! Depcrate_um_setupapiother_37874 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37874"}
// Dependencies: {}
extern "system" { pub fn SetupWriteTextLogInfLine (LogToken : SP_LOG_TOKEN , Flags : DWORD , InfHandle : HINF , Context : PINFCONTEXT ,) -> () ; pub fn SetupGetBackupInformationA (QueueHandle : HSPFILEQ , BackupParams : PSP_BACKUP_QUEUE_PARAMS_A ,) -> BOOL ; pub fn SetupGetBackupInformationW (QueueHandle : HSPFILEQ , BackupParams : PSP_BACKUP_QUEUE_PARAMS_W ,) -> BOOL ; pub fn SetupPrepareQueueForRestoreA (QueueHandle : HSPFILEQ , BackupPath : PCSTR , RestoreFlags : DWORD ,) -> BOOL ; pub fn SetupPrepareQueueForRestoreW (QueueHandle : HSPFILEQ , BackupPath : PCWSTR , RestoreFlags : DWORD ,) -> BOOL ; pub fn SetupSetNonInteractiveMode (NonInteractiveFlag : BOOL ,) -> BOOL ; pub fn SetupGetNonInteractiveMode () -> BOOL ; pub fn SetupDiCreateDeviceInfoList (ClassGuid : * const GUID , hwndParent : HWND ,) -> HDEVINFO ; pub fn SetupDiCreateDeviceInfoListExA (ClassGuid : * const GUID , hwndParent : HWND , MachineName : PCSTR , Reserved : PVOID ,) -> HDEVINFO ; pub fn SetupDiCreateDeviceInfoListExW (ClassGuid : * const GUID , hwndParent : HWND , MachineName : PCWSTR , Reserved : PVOID ,) -> HDEVINFO ; pub fn SetupDiGetDeviceInfoListClass (DeviceInfoSet : HDEVINFO , ClassGuid : LPGUID ,) -> BOOL ; pub fn SetupDiGetDeviceInfoListDetailA (DeviceInfoSet : HDEVINFO , DeviceInfoSetDetailData : PSP_DEVINFO_LIST_DETAIL_DATA_A ,) -> BOOL ; pub fn SetupDiGetDeviceInfoListDetailW (DeviceInfoSet : HDEVINFO , DeviceInfoSetDetailData : PSP_DEVINFO_LIST_DETAIL_DATA_W ,) -> BOOL ; }
};
}
