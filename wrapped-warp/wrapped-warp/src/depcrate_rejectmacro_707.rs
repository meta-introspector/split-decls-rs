// Generated macro for macro_707 (macro)
macro_rules! Depcrate_rejectmacro_707 {
() => {
// Module: crate::reject
// Provides: {"macro_707"}
// Dependencies: {}
enum_known ! { MethodNotAllowed (MethodNotAllowed) , InvalidHeader (InvalidHeader) , MissingHeader (MissingHeader) , MissingCookie (MissingCookie) , InvalidQuery (InvalidQuery) , LengthRequired (LengthRequired) , PayloadTooLarge (PayloadTooLarge) , UnsupportedMediaType (UnsupportedMediaType) , FileOpenError (crate :: fs :: FileOpenError) , FilePermissionError (crate :: fs :: FilePermissionError) , BodyReadError (crate :: filters :: body :: BodyReadError) , BodyDeserializeError (crate :: filters :: body :: BodyDeserializeError) , CorsForbidden (crate :: cors :: CorsForbidden) , # [cfg (feature = "websocket")] MissingConnectionUpgrade (crate :: ws :: MissingConnectionUpgrade) , MissingExtension (crate :: ext :: MissingExtension) , BodyConsumedMultipleTimes (crate :: filters :: body :: BodyConsumedMultipleTimes) , }
};
}
