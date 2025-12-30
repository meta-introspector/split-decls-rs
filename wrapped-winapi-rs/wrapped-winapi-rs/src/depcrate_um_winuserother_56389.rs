// Generated macro for other_56389 (other)
macro_rules! Depcrate_um_winuserother_56389 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56389"}
// Dependencies: {}
extern "system" { pub fn LoadImageA (hInst : HINSTANCE , name : LPCSTR , type_ : UINT , cx : c_int , cy : c_int , fuLoad : UINT ,) -> HANDLE ; pub fn LoadImageW (hInst : HINSTANCE , name : LPCWSTR , type_ : UINT , cx : c_int , cy : c_int , fuLoad : UINT ,) -> HANDLE ; pub fn CopyImage (h : HANDLE , type_ : UINT , cx : c_int , cy : c_int , flags : UINT ,) -> HANDLE ; }
};
}
