// Generated macro for inappropriate_message (function)
macro_rules! Depcrate_checkinappropriate_message {
() => {
// Module: crate::check
// Provides: {"inappropriate_message"}
// Dependencies: {}
pub (crate) fn inappropriate_message (payload : & MessagePayload < '_ > , content_types : & [ContentType] ,) -> Error { warn ! ("Received a {:?} message while expecting {content_types:?}" , payload . content_type () ,) ; Error :: InappropriateMessage { expect_types : content_types . to_vec () , got_type : payload . content_type () , } }
};
}
