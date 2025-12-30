// Generated macro for other_56390 (other)
macro_rules! Depcrate_um_winuserother_56390 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56390"}
// Dependencies: {}
extern "system" { pub fn DrawIconEx (hdc : HDC , xLeft : c_int , yTop : c_int , hIcon : HICON , cxWidth : c_int , cyWidth : c_int , istepIfAniCur : UINT , hbrFlickerFreeDraw : HBRUSH , diFlags : UINT ,) -> BOOL ; pub fn CreateIconIndirect (piconinfo : PICONINFO ,) -> HICON ; pub fn CopyIcon (hIcon : HICON ,) -> HICON ; pub fn GetIconInfo (hIcon : HICON , piconinfo : PICONINFO ,) -> BOOL ; }
};
}
