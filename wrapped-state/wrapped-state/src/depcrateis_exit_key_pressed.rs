// Generated macro for is_exit_key_pressed (function)
macro_rules! Depcrateis_exit_key_pressed {
() => {
// Module: crate
// Provides: {"is_exit_key_pressed"}
// Dependencies: {}
pub fn is_exit_key_pressed () -> std :: io :: Result < bool > { Ok (event :: read () ? . as_key_press_event () . is_some_and (| key | matches ! (key . code , KeyCode :: Esc | KeyCode :: Char ('q')))) }
};
}
