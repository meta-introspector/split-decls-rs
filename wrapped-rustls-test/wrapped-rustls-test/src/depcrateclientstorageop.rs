// Generated macro for ClientStorageOp (enum)
macro_rules! DepcrateClientStorageOp {
() => {
// Module: crate
// Provides: {"ClientStorageOp"}
// Dependencies: {}
# [derive (Debug , Clone)] # [allow (dead_code)] pub enum ClientStorageOp { SetKxHint (ServerName < 'static > , NamedGroup) , GetKxHint (ServerName < 'static > , Option < NamedGroup >) , SetTls12Session (ServerName < 'static >) , GetTls12Session (ServerName < 'static > , bool) , RemoveTls12Session (ServerName < 'static >) , InsertTls13Ticket (ServerName < 'static >) , TakeTls13Ticket (ServerName < 'static > , bool) , }
};
}
