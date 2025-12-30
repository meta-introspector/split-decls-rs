// Generated macro for other_55816 (other)
macro_rules! Depcrate_um_winuserother_55816 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55816"}
// Dependencies: {}
extern "system" { pub fn InitializeTouchInjection (maxCount : UINT32 , dwMode : DWORD ,) -> BOOL ; pub fn InjectTouchInput (count : UINT32 , contacts : * const POINTER_TOUCH_INFO ,) -> BOOL ; }
};
}
