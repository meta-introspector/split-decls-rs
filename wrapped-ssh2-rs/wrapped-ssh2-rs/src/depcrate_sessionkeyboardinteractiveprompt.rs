// Generated macro for KeyboardInteractivePrompt (trait)
macro_rules! Depcrate_sessionKeyboardInteractivePrompt {
() => {
// Module: crate::session
// Provides: {"KeyboardInteractivePrompt"}
// Dependencies: {}
# [doc = " Called by libssh2 to respond to some number of challenges as part of"] # [doc = " keyboard interactive authentication."] pub trait KeyboardInteractivePrompt { # [doc = " `username` is the user name to be authenticated. It may not be the"] # [doc = " same as the username passed to `Session::userauth_keyboard_interactive`,"] # [doc = " and may be empty."] # [doc = " `instructions` is some informational text to be displayed to the user."] # [doc = " `prompts` is a series of prompts (or challenges) that must be responded"] # [doc = " to."] # [doc = " The return value should be a Vec that holds one response for each prompt."] fn prompt < 'a > (& mut self , username : & str , instructions : & str , prompts : & [Prompt < 'a >] ,) -> Vec < String > ; }
};
}
