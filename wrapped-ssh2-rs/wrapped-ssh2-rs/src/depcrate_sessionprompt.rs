// Generated macro for Prompt (struct)
macro_rules! Depcrate_sessionPrompt {
() => {
// Module: crate::session
// Provides: {"Prompt"}
// Dependencies: {}
# [doc = " A prompt/challenge returned as part of keyboard-interactive authentication"] # [derive (Debug)] pub struct Prompt < 'a > { # [doc = " The label to show when prompting the user"] pub text : Cow < 'a , str > , # [doc = " If true, the response that the user inputs should be displayed"] # [doc = " as they type.  If false then treat it as a password entry and"] # [doc = " do not display what is typed in response to this prompt."] pub echo : bool , }
};
}
