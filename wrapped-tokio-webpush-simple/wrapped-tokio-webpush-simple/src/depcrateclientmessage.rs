// Generated macro for ClientMessage (enum)
macro_rules! DepcrateClientMessage {
() => {
// Module: crate
// Provides: {"ClientMessage"}
// Dependencies: {}
# [derive (Deserialize)] # [serde (tag = "messageType" , rename_all = "lowercase")] enum ClientMessage { Hello { uaid : Option < Uuid > , # [serde (rename = "channelIDs" , skip_serializing_if = "Option::is_none")] channel_ids : Option < Vec < Uuid > > , # [serde (skip_serializing_if = "Option::is_none")] use_webpush : Option < bool > , } , Register { # [serde (rename = "channelID")] channel_id : Uuid , } , Unregister { # [serde (rename = "channelID")] channel_id : Uuid , } , }
};
}
