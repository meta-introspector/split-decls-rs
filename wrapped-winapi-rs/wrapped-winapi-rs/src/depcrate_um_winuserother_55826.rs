// Generated macro for other_55826 (other)
macro_rules! Depcrate_um_winuserother_55826 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55826"}
// Dependencies: {}
extern "system" { pub fn CreateSyntheticPointerDevice (pointerType : POINTER_INPUT_TYPE , maxCount : ULONG , mode : POINTER_FEEDBACK_MODE ,) -> HSYNTHETICPOINTERDEVICE ; pub fn InjectSyntheticPointerInput (device : HSYNTHETICPOINTERDEVICE , pointerInfo : * const POINTER_TYPE_INFO , count : UINT32 ,) -> BOOL ; pub fn DestroySyntheticPointerDevice (device : HSYNTHETICPOINTERDEVICE ,) ; }
};
}
