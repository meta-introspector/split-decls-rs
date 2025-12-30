// Generated macro for PreEncryptAction (enum)
macro_rules! Depcrate_record_layerPreEncryptAction {
() => {
// Module: crate::record_layer
// Provides: {"PreEncryptAction"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] pub (crate) enum PreEncryptAction { # [doc = " No action is needed before calling `encrypt_outgoing`"] Nothing , # [doc = " A `key_update` request should be sent ASAP."] # [doc = ""] # [doc = " If that is not possible (for example, the connection is TLS1.2), a `close_notify`"] # [doc = " alert should be sent instead."] RefreshOrClose , # [doc = " Do not call `encrypt_outgoing` further, it will panic rather than"] # [doc = " over-use the key."] Refuse , }
};
}
