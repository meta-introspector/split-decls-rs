// Generated macro for other_56350 (other)
macro_rules! Depcrate_um_winuserother_56350 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56350"}
// Dependencies: {}
extern "system" { pub fn LoadBitmapA (hInstance : HINSTANCE , lpBitmapName : LPCSTR ,) -> HBITMAP ; pub fn LoadBitmapW (hInstance : HINSTANCE , lpBitmapName : LPCWSTR ,) -> HBITMAP ; pub fn LoadCursorA (hInstance : HINSTANCE , lpCursorName : LPCSTR ,) -> HCURSOR ; pub fn LoadCursorW (hInstance : HINSTANCE , lpCursorName : LPCWSTR ,) -> HCURSOR ; pub fn LoadCursorFromFileA (lpFileName : LPCSTR ,) -> HCURSOR ; pub fn LoadCursorFromFileW (lpFileName : LPCWSTR ,) -> HCURSOR ; pub fn CreateCursor (hInst : HINSTANCE , xHotSpot : c_int , yHotSpot : c_int , nWidth : c_int , nHeight : c_int , pvAndPlane : * const VOID , pvXORPlane : * const VOID ,) -> HCURSOR ; pub fn DestroyCursor (hCursor : HCURSOR ,) -> BOOL ; }
};
}
