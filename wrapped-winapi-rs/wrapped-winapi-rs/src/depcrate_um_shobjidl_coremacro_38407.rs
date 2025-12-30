// Generated macro for macro_38407 (macro)
macro_rules! Depcrate_um_shobjidl_coremacro_38407 {
() => {
// Module: crate::um::shobjidl_core
// Provides: {"macro_38407"}
// Dependencies: {}
RIDL ! { # [uuid (0xea1afb91 , 0x9e28 , 0x4b86 , 0x90 , 0xe9 , 0x9e , 0x9f , 0x8a , 0x5e , 0xef , 0xaf)] interface ITaskbarList3 (ITaskbarList3Vtbl) : ITaskbarList2 (ITaskbarList2Vtbl) { fn SetProgressValue (hwnd : HWND , ullCompleted : ULONGLONG , ullTotal : ULONGLONG ,) -> HRESULT , fn SetProgressState (hwnd : HWND , tbpFlags : TBPFLAG ,) -> HRESULT , fn RegisterTab (hwndTab : HWND , hwndMDI : HWND ,) -> HRESULT , fn UnregisterTab (hwndTab : HWND ,) -> HRESULT , fn SetTabOrder (hwndTab : HWND , hwndInsertBefore : HWND ,) -> HRESULT , fn SetTabActive (hwndTab : HWND , hwndMDI : HWND , dwReserved : DWORD ,) -> HRESULT , fn ThumbBarAddButtons (hwnd : HWND , cButtons : UINT , pButton : LPTHUMBBUTTON ,) -> HRESULT , fn ThumbBarUpdateButtons (hwnd : HWND , cButtons : UINT , pButton : LPTHUMBBUTTON ,) -> HRESULT , fn ThumbBarSetImageList (hwnd : HWND , himl : HIMAGELIST ,) -> HRESULT , fn SetOverlayIcon (hwnd : HWND , hIcon : HICON , pszDescription : LPCWSTR ,) -> HRESULT , fn SetThumbnailTooltip (hwnd : HWND , pszTip : LPCWSTR ,) -> HRESULT , fn SetThumbnailClip (hwnd : HWND , prcClip : * mut RECT ,) -> HRESULT , } }
};
}
