// Generated macro for macro_121 (macro)
macro_rules! Depcrate_buffer_workermacro_121 {
() => {
// Module: crate::buffer::worker
// Provides: {"macro_121"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " Task that handles processing the buffer. This type should not be used"] # [doc = " directly, instead `Buffer` requires an `Executor` that can accept this task."] # [doc = ""] # [doc = " The struct is `pub` in the private module and the type is *not* re-exported"] # [doc = " as part of the public API. This is the \"sealed\" pattern to include \"private\""] # [doc = " types in public traits that are not meant for consumers of the library to"] # [doc = " implement (only call)."] # [derive (Debug)] pub struct Worker < T , Request > where T : Service < Request >, { current_message : Option < Message < Request , T :: Future >>, rx : mpsc :: Receiver < Message < Request , T :: Future >>, service : T , finish : bool , failed : Option < ServiceError >, handle : Handle , } }
};
}
