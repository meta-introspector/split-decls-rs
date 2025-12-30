// Generated macro for impl_40 (impl)
macro_rules! Depcrate_ansiimpl_40 {
() => {
// Module: crate::ansi
// Provides: {"impl_40"}
// Dependencies: {}
impl Rgb { # [doc = " Implementation of [W3C's luminance algorithm]."] # [doc = ""] # [doc = " [W3C's luminance algorithm]: https://www.w3.org/TR/WCAG20/#relativeluminancedef"] # [cfg (feature = "std")] pub fn luminance (self) -> f64 { let channel_luminance = | channel | { let channel = channel as f64 / 255. ; if channel <= 0.03928 { channel / 12.92 } else { f64 :: powf ((channel + 0.055) / 1.055 , 2.4) } } ; let r_luminance = channel_luminance (self . r) ; let g_luminance = channel_luminance (self . g) ; let b_luminance = channel_luminance (self . b) ; 0.2126 * r_luminance + 0.7152 * g_luminance + 0.0722 * b_luminance } # [doc = " Implementation of [W3C's contrast algorithm]."] # [doc = ""] # [doc = " [W3C's contrast algorithm]: https://www.w3.org/TR/WCAG20/#contrast-ratiodef"] # [cfg (feature = "std")] pub fn contrast (self , other : Rgb) -> f64 { let self_luminance = self . luminance () ; let other_luminance = other . luminance () ; let (darker , lighter) = if self_luminance > other_luminance { (other_luminance , self_luminance) } else { (self_luminance , other_luminance) } ; (lighter + 0.05) / (darker + 0.05) } }
};
}
