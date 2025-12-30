// Generated macro for temperature_style (function)
macro_rules! Depcratetemperature_style {
() => {
// Module: crate
// Provides: {"temperature_style"}
// Dependencies: {}
# [doc = " create a yellow to red value based on the value (50-90)"] fn temperature_style (value : u8) -> Style { let green = (255.0 * (1.0 - f64 :: from (value - 50) / 40.0)) as u8 ; let color = Color :: Rgb (255 , green , 0) ; Style :: new () . fg (color) }
};
}
