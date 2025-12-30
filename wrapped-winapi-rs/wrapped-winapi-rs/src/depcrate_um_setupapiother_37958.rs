// Generated macro for other_37958 (other)
macro_rules! Depcrate_um_setupapiother_37958 {
() => {
// Module: crate::um::setupapi
// Provides: {"other_37958"}
// Dependencies: {}
extern "system" { pub fn SetupDiDrawMiniIcon (hdc : HDC , rc : RECT , MiniIconIndex : INT , Flags : DWORD ,) -> INT ; pub fn SetupDiGetClassBitmapIndex (ClassGuid : * const GUID , MiniIconIndex : PINT ,) -> BOOL ; pub fn SetupDiGetClassImageList (ClassImageListData : PSP_CLASSIMAGELIST_DATA ,) -> BOOL ; pub fn SetupDiGetClassImageListExA (ClassImageListData : PSP_CLASSIMAGELIST_DATA , MachineName : PCSTR , Reserved : PVOID ,) -> BOOL ; pub fn SetupDiGetClassImageListExW (ClassImageListData : PSP_CLASSIMAGELIST_DATA , MachineName : PCWSTR , Reserved : PVOID ,) -> BOOL ; pub fn SetupDiGetClassImageIndex (ClassImageListData : PSP_CLASSIMAGELIST_DATA , ClassGuid : * const GUID , ImageIndex : PINT ,) -> BOOL ; pub fn SetupDiDestroyClassImageList (ClassImageListData : PSP_CLASSIMAGELIST_DATA ,) -> BOOL ; }
};
}
