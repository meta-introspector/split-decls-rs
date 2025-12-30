// Generated macro for error_from_exception (function)
macro_rules! Depcrate_thread_atomicserror_from_exception {
() => {
// Module: crate::thread::atomics
// Provides: {"error_from_exception"}
// Dependencies: {}
# [doc = " Convert a [`JsValue`] to an [`DomException`] and then to an [`Error`]."] # [cfg (any (feature = "audio-worklet" , feature = "message"))] fn error_from_exception (error : JsValue) -> Error { let error : DomException = error . unchecked_into () ; Error :: other (format ! ("{}: {}" , error . name () , error . message ())) }
};
}
