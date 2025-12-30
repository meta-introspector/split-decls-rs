// Generated macro for vertical_bar (function)
macro_rules! Depcratevertical_bar {
() => {
// Module: crate
// Provides: {"vertical_bar"}
// Dependencies: {}
fn vertical_bar (hour : usize , temperature : & u8) -> Bar < '_ > { Bar :: default () . value (u64 :: from (* temperature)) . label (Line :: from (format ! ("{hour:>02}:00"))) . text_value (format ! ("{temperature:>3}°")) . style (temperature_style (* temperature)) . value_style (temperature_style (* temperature) . reversed ()) }
};
}
