// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
# [async_trait] impl key_value_store_server :: KeyValueStore for ServerImpl { async fn get (& self , request : Request < GetRequest >) -> Result < Response < GetReply > , Status > { let key = request . into_inner () . key ; if let Some (value) = self . db . read () . unwrap () . get (& key) . cloned () { let reply = GetReply { value : value . to_vec () , } ; Ok (Response :: new (reply)) } else { Err (Status :: not_found ("key not found")) } } async fn set (& self , request : Request < SetRequest >) -> Result < Response < SetReply > , Status > { let SetRequest { key , value } = request . into_inner () ; let value = Bytes :: from (value) ; let _send = self . tx . send (SubscribeReply { key : key . clone () }) ; self . db . write () . unwrap () . insert (key , value) ; Ok (Response :: new (SetReply { })) } type SubscribeStream = Pin < Box < dyn Stream < Item = Result < SubscribeReply , Status > > + Send + Sync + 'static > > ; async fn subscribe (& self , request : Request < SubscribeRequest > ,) -> Result < Response < Self :: SubscribeStream > , Status > { let SubscribeRequest { } = request . into_inner () ; let rx = self . tx . subscribe () ; let stream = BroadcastStream :: new (rx) . filter_map (| item | async move { item . ok () }) . map (Ok) ; let stream = Box :: pin (stream) as Self :: SubscribeStream ; let res = Response :: new (stream) ; Ok (res) } }
};
}
