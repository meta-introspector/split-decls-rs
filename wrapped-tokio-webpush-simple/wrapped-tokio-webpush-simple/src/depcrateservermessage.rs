// Generated macro for ServerMessage (enum)
macro_rules! DepcrateServerMessage {
() => {
// Module: crate
// Provides: {"ServerMessage"}
// Dependencies: {}
# [derive (Serialize)] # [serde (tag = "messageType" , rename_all = "lowercase")] enum ServerMessage { Hello { uaid : Uuid , status : u32 , # [serde (skip_serializing_if = "Option::is_none")] use_webpush : Option < bool > , } , Register { # [serde (rename = "channelID")] channel_id : Uuid , status : u32 , # [serde (rename = "pushEndpoint")] push_endpoint : String , } , Unregister { # [serde (rename = "channelID")] channel_id : Uuid , status : u32 , } , Notification (Notification) , }
};
}
