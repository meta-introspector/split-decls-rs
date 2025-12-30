// Generated macro for convert_temp_celsius (function)
macro_rules! Depcrate_unix_linux_componentconvert_temp_celsius {
() => {
// Module: crate::unix::linux::component
// Provides: {"convert_temp_celsius"}
// Dependencies: {}
# [doc = " Takes a raw temperature in mili-celsius and convert it to celsius."] # [inline] fn convert_temp_celsius (temp : Option < i32 >) -> Option < f32 > { temp . map (| n | (n as f32) / 1000f32) }
};
}
