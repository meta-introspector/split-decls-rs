// Generated macro for ServerImpl (struct)
macro_rules! DepcrateServerImpl {
() => {
// Module: crate
// Provides: {"ServerImpl"}
// Dependencies: {}
# [derive (Debug , Clone)] struct ServerImpl { db : Arc < RwLock < HashMap < String , Bytes > > > , tx : Sender < SubscribeReply > , }
};
}
