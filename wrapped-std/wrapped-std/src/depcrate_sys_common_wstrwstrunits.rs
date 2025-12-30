// Generated macro for WStrUnits (struct)
macro_rules! Depcrate_sys_common_wstrWStrUnits {
() => {
// Module: crate::sys_common::wstr
// Provides: {"WStrUnits"}
// Dependencies: {}
# [doc = " A safe iterator over a LPWSTR"] # [doc = " (aka a pointer to a series of UTF-16 code units terminated by a NULL)."] pub struct WStrUnits < 'a > { lpwstr : NonNull < u16 > , lifetime : PhantomData < & 'a [u16] > , }
};
}
