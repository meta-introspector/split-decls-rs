// Generated macro for Message (enum)
macro_rules! DepcrateMessage {
() => {
// Module: crate
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Message wrapper enum used to pass through the scheduler to the Controller or View"] pub enum Message { # [doc = " Message wrapper to send to the controller"] Controller (ControllerMessage) , # [doc = " Message wrapper to send to the view"] View (ViewMessage) , }
};
}
