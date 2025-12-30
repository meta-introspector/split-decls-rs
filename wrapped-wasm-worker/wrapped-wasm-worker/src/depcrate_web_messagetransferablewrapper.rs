// Generated macro for TransferableWrapper (struct)
macro_rules! Depcrate_web_messageTransferableWrapper {
() => {
// Module: crate::web::message
// Provides: {"TransferableWrapper"}
// Dependencies: {}
# [doc = " Wrapper that implements [`MessageSend`] for values implementing"] # [doc = " [`Transferable`]."] # [doc = ""] # [doc = " For a more complete documentation see"] # [doc = " [`web::spawn_with_message()`](super::spawn_with_message)."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct TransferableWrapper < T > (pub T) where T : Into < JsValue > + JsCast + Transferable ;
};
}
