// Generated macro for App (struct)
macro_rules! DepcrateApp {
() => {
// Module: crate
// Provides: {"App"}
// Dependencies: {}
# [doc = " App holds the state of the application"] struct App { # [doc = " Current value of the input box"] input : String , # [doc = " Position of cursor in the editor area."] character_index : usize , # [doc = " Current input mode"] input_mode : InputMode , # [doc = " History of recorded messages"] messages : Vec < String > , }
};
}
